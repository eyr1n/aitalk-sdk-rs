use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use aitalk_sdk::{
    AiTalkSdk, Engine, EngineLicense, Tts, TtsParameter, VoiceDictionaryMetaContainer,
};
use anyhow::Result;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::Parser;
use hound::{SampleFormat, WavSpec, WavWriter};
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Fft, FixedSync, Resampler};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 3000)]
    port: u16,
    #[arg(long, env = "AIVOICE_DIR")]
    aivoice_dir: PathBuf,
    #[arg(long, env = "AITALK_LICENSE_CODE", hide_env_values = true)]
    license_code: String,
    #[arg(long, env = "AITALK_PRODUCT", hide_env_values = true)]
    product: String,
}

struct Voice {
    engine: &'static Engine,
    tts: Mutex<Tts>,
    sample_rate: u32,
}

#[derive(Deserialize)]
struct SynthesizeRequest {
    text: String,
    voice: String,
    sample_rate: Option<u32>,
    parameters: Option<Parameters>,
}

#[derive(Deserialize)]
struct Parameters {
    middle_pause: Option<i32>,
    long_pause: Option<i32>,
    sentence_pause: Option<i32>,
    volume: Option<f64>,
    speed: Option<f64>,
    pitch: Option<f64>,
    pitch_range: Option<f64>,
    styles: Option<BTreeMap<String, f64>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let args = Args::parse();
    let editor_dir = args.aivoice_dir.join("AIVoiceEditor");
    let sdk = unsafe { AiTalkSdk::new(editor_dir.join("AITalk_SDK.dll")) }?;
    let engine = Engine::init_once(
        sdk,
        &editor_dir,
        EngineLicense {
            path: editor_dir.join("aitalk5.lic"),
            code: args.license_code,
            values: vec![(3, args.product)],
        },
    )?;

    let voices = Arc::new(load_voices(engine, &args.aivoice_dir)?);
    let app = Router::new()
        .route("/voices", get(get_voices))
        .route("/synthesize", post(synthesize))
        .with_state(voices);

    let listener = TcpListener::bind((args.host.as_str(), args.port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn load_voices(engine: &'static Engine, aivoice_dir: &Path) -> Result<BTreeMap<String, Voice>> {
    let lang_dic = aivoice_dir.join(r"Lang\standard\standard.aildic");
    let voice_dir = aivoice_dir.join("Voice");
    let mut voices = BTreeMap::new();

    for entry in fs::read_dir(&voice_dir)? {
        let entry = entry?;
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        let path = entry.path();
        let voice_dic = path.join(format!("{name}.aivdic"));
        if !voice_dic.is_file() {
            continue;
        }

        let sample_rate =
            u32::try_from(VoiceDictionaryMetaContainer::new(engine, &voice_dic)?.rate()?)?;
        let mut tts = Tts::new(engine)?;
        tts.put_language_dictionary("standard", &lang_dic)?;
        tts.put_voice_dictionary_license(&name, &path.join("voice.lic"))?;
        tts.put_voice_dictionary(&name, &voice_dic)?;
        tts.select_language_dictionary("standard")?;
        tts.select_voice_dictionary(&name)?;

        voices.insert(
            name,
            Voice {
                engine,
                tts: Mutex::new(tts),
                sample_rate,
            },
        );
    }
    Ok(voices)
}

async fn get_voices(State(voices): State<Arc<BTreeMap<String, Voice>>>) -> Json<Vec<String>> {
    Json(voices.keys().cloned().collect())
}

async fn synthesize(
    State(voices): State<Arc<BTreeMap<String, Voice>>>,
    Json(request): Json<SynthesizeRequest>,
) -> Result<Response, StatusCode> {
    if !voices.contains_key(&request.voice) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let wav = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let voice = &voices[&request.voice];
        let mut tts = voice.tts.lock().unwrap().duplicate()?;
        if let Some(parameters) = &request.parameters {
            let parameter = tts_parameter(voice.engine, parameters)?;
            tts.put_parameter("custom", &parameter)?;
            tts.select_parameter("custom")?;
        }

        let pcm = tts.run(&request.text)?;

        let sample_rate = request.sample_rate.unwrap_or(voice.sample_rate);
        let pcm = if sample_rate == voice.sample_rate {
            pcm
        } else {
            resample_pcm(&pcm, voice.sample_rate, sample_rate)?
        };
        Ok(encode_wav(&pcm, sample_rate)?)
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(([(header::CONTENT_TYPE, "audio/wav")], wav).into_response())
}

fn tts_parameter(
    engine: &'static Engine,
    parameters: &Parameters,
) -> aitalk_sdk::Result<TtsParameter> {
    let mut parameter = TtsParameter::new(engine)?;
    if let Some(middle_pause) = parameters.middle_pause {
        parameter.set_wait_time_middle_text(middle_pause)?;
    }
    if let Some(long_pause) = parameters.long_pause {
        parameter.set_wait_time_long_text(long_pause)?;
    }
    if let Some(sentence_pause) = parameters.sentence_pause {
        parameter.set_wait_time_eos(sentence_pause)?;
    }
    if let Some(volume) = parameters.volume {
        parameter.set_volume(volume)?;
    }
    if let Some(speed) = parameters.speed {
        parameter.set_rate(speed)?;
    }
    if let Some(pitch) = parameters.pitch {
        parameter.set_pitch(pitch)?;
    }
    if let Some(pitch_range) = parameters.pitch_range {
        parameter.set_emphasis(pitch_range)?;
    }
    if let Some(styles) = &parameters.styles {
        parameter.set_style_color(&style_color(styles))?;
    }
    Ok(parameter)
}

fn style_color(styles: &BTreeMap<String, f64>) -> String {
    styles
        .iter()
        .map(|(name, value)| format!("{name}:{value:.2}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn resample_pcm(pcm: &[i16], from: u32, to: u32) -> Result<Vec<i16>> {
    let input: Vec<f64> = pcm
        .iter()
        .map(|&sample| f64::from(sample) / 32768.0)
        .collect();
    let mut resampler = Fft::<f64>::new(from as usize, to as usize, 1024, 1, FixedSync::Both)?;
    let adapter = InterleavedSlice::new(input.as_slice(), 1, input.len())?;
    let output = resampler.process_all(&adapter, input.len(), None)?;
    Ok(output
        .take_data()
        .into_iter()
        .map(|sample| (sample * 32768.0).round().clamp(-32768.0, 32767.0) as i16)
        .collect())
}

fn encode_wav(pcm: &[i16], sample_rate: u32) -> hound::Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut buffer = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut buffer, spec)?;
    for &sample in pcm {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    Ok(buffer.into_inner())
}

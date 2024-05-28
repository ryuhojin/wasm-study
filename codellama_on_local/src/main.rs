use tch::{nn, Device, Tensor};
use std::path::Path;

fn main() {
    // 모델 파일 경로 설정
    let model_path = Path::new("코드라마 경로");

    // 모델 로드
    let var_store = nn::VarStore::new(Device::Cpu);
    
}
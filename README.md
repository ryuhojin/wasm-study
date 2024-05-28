# WASM-STUDY For Edge Computing

목표 : WASM + RUST + EDGE COMPUTING
자원 많이 먹는 AI나 고비용 작업들을 클라이언트에서 처리하기 위한 학습


## FIRST. CODELLAMA ON LOCAL
1. 모델 다운로드 : https://llama.meta.com/llama-downloads/
2. 해당 URL에서 라이선스 신청하고 코드라마 체크 후 약관 잘 읽어보고 URL을 받는다.
3. https://github.com/meta-llama/codellama/
4. 코드라마 URL로 접근 후 download.sh실행
5. URL에 제공받은 키값이 포함된 URL을 넣고 실행 하고 모델을 정한다.
6. 로컬 개발망은 컴퓨터 성능이 대체로 좋다고 볼 수 없기 때문에 7B-Inst버전을 이용한다.
7. 다운받은 모델을 rust의 pytorch 크레이트를 이용하여 실행시켜 준다.

/*
[12] 프로젝트: 명령줄 프로그램 작성하기
std::env::args 함수를 사용해야 한다. 이 함수는 minigrep 도구에 전달된 명령줄 인수의 반복자를 리턴한다.
반복자는 일련의 값을 생성하며 collect 메서드를 이용해 생성된 값들을 벡터 같은 컬렉션으로 변환할 수 있다는 점만 알아두면 된다.
먼저, args 함수를 사용할 수 있도록 use 구문을 이용해 std::env 모듈을 범위로 가져온다. std::env::args 함수는
두 개의 중첩된 모듈에 선언되어 있다.
env::args 함수를 호출한 후 곧바로 collect 메서드를 호출해서 반복자가 생성하는 모든 값을 벡터로 변환한다.
collect함수는 다양한 종류의 컬렉션을 생성할 수 있으므로 args 변수의 타입을 문자열의 벡터로 명시해야 한다.
러스트에서는 타입을 명시해야 할 일이 거의 없지만, collect 함수는 여러 종류의 컬렉션을 생성하므로 러스트가 원하는 타입을 추측할 수 없어서
원하는 타입을 반드시 명시해야 하는 몇 안 되는 함수 중 하나다.

마지막으로 디버그 형식(:?)을 이용해 벡터의 값을 출력한다. 여기까지 코드를 작성했으면 인수를 지정하지 않고 한번 실행해 보자.
12.1.2 인숫값을 변수에 저장하기
인수의 벡터에 저장된 값을 출력한 것은 단지 프로그램이 명령줄 인수에 지정된 값에 접근할 수 있음을 보여주기 위한 것이다. 이제는 두 인수의 값을
사용할 수 있도록 변수에 저장해야한다.

12.3 모듈화와 에러 처리 향상을 위한 리팩토링

 */
use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = Config::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}

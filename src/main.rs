// src/main.rs
#![no_std]
#![no_main]

mod vga_buffer; // VGA 출력 모듈 등록

use core::panic::PanicInfo;

/// 부트로더의 매핑 매크로를 쓰지 않고, CPU가 바이오스 통과 후 가장 먼저 실행하는 
/// 순수 하드웨어 진입점(_start)을 직접 선언합니다.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 부트로더의 가상 메모리 테이블 간섭 없이, 
    // 하드웨어 물리 주소 0xb8000 영역에 직접 글자를 밀어 넣습니다.
    vga_buffer::print_dos_prompt("Starting MS-DOS...\n\nC:\\>");
    
    // CPU가 다음 명령어로 넘어가지 않고 이 자리에 안전하게 멈추도록 홀트(hlt) 루프를 돕니다.
    loop {
        x86_64::instructions::hlt();
    }
}

/// 만약 이 안에서 다른 에러가 나더라도 무한 루프를 돌며 화면을 유지하는 패닉 핸들러
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

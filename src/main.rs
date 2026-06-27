#![no_std]
#![no_main]

// 1. 데이터 타입 정의 부에 8바이트 정렬을 명시합니다.
#[repr(C, align(8))]
pub struct Multiboot2Header {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_tag_type: u16,
    end_tag_flags: u16,
    end_tag_size: u32,
}

// 2. static 인스턴스 생성 (여기서는 #[repr]을 제거합니다)
#[link_section = ".multiboot"]
#[no_mangle]
pub static MULTIBOOT_HEADER: Multiboot2Header = Multiboot2Header {
    magic: 0xE85250D6,
    architecture: 0,
    header_length: core::mem::size_of::<Multiboot2Header>() as u32,
    checksum: !(0xE85250D6 + 0 + core::mem::size_of::<Multiboot2Header>() as u32) + 1,
    end_tag_type: 0,
    end_tag_flags: 0,
    end_tag_size: 8,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// -------------------------------------------------------------
// 🚀 커널 진짜 진입점 및 화면 출력 복구부
// -------------------------------------------------------------
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. 레거시 VGA 텍스트 모드 메모리 시작 주소 (0xB8000)
    let vga_buffer = 0xb8000 as *mut u16;

    // 2. 출력할 MS-DOS 감성 문자열
    let msg1 = b"Starting MS-DOS...";
    let msg2 = b"C:\\>";

    // 3. 화면 초기화 (검은 바탕에 흰색 공백으로 전체 밀기)
    // VGA 텍스트 화면 크기는 가로 80글자 * 세로 25줄 = 총 2000글자 공간입니다.
    unsafe {
        for i in 0..2000 {
            // 0x0720 -> 0x07(검은 바탕/회백색 글씨 색상 옵션) + 0x20(공백 문자 ' ')
            *vga_buffer.offset(i) = 0x0720;
        }
    }

    // 4. 첫 번째 줄에 "Starting MS-DOS..." 출력
    // VGA 메모리는 [색상 1바이트 + 문자 1바이트]가 합쳐진 u16 포맷입니다.
    for (i, &byte) in msg1.iter().enumerate() {
        unsafe {
            // 0x0F00은 밝은 흰색 글씨 속성입니다.
            *vga_buffer.offset(i as isize) = (0x0F00 | byte as u16);
        }
    }

    // 5. 세 번째 줄(인덱스 80 * 2 = 160번째 칸)에 "C:\>" 프롬프트 출력
    for (i, &byte) in msg2.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((80 * 2 + i) as isize) = (0x0F00 | byte as u16);
        }
    }

    // 작업 완료 후 CPU 대기 상태 유지
    loop {}
}


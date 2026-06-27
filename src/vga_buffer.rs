// src/vga_buffer.rs

/// VGA 텍스트 모드에서 사용할 수 있는 표준 16가지 색상 정의
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,      Blue = 1,         Green = 2,       Cyan = 3,
    Red = 4,        Magenta = 5,      Brown = 6,       LightGray = 7,
    DarkGray = 8,   LightBlue = 9,    LightGreen = 10, LightCyan = 11,
    LightRed = 12,  Pink = 13,        Yellow = 14,     White = 15,
}

/// 글자 색상과 배경 색상을 1바이트 크기로 결합하는 구조체
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// 화면에 표시될 실제 글자 한 칸의 구조 (ASCII 문자 1바이트 + 색상 코드 1바이트 = 총 2바이트)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// 표준 VGA 텍스트 모드의 화면 크기 정의 (가로 80칸, 세로 25줄)
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// 하드웨어 VGA 메모리 영역을 가리키는 구조체
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// 화면 출력 상태(현재 커서 위치, 색상, 메모리 주소)를 관리하는 관리자
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// 1바이트 ASCII 문자를 화면에 출력하는 함수
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // 개행 문자 처리
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // 하드웨어 버퍼의 해당 위치에 글자와 색상 직접 각인
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// 문자열을 반복하며 바이트 단위로 출력하는 함수
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 출력 가능한 ASCII 범위 또는 개행 문자만 허용
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // 출력 불가능한 문자는 네모(■) 모양 기호로 대체
            }
        }
    }

    /// 화면이 꽉 차거나 개행(\n) 시 모든 행을 한 줄씩 위로 올리는 스크롤 기능
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// 특정 행을 공백으로 깨끗하게 비우는 함수
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

/// 외부(main.rs)에서 편리하게 프롬프트 텍스트를 출력할 수 있도록 제공하는 공개 함수
pub fn print_dos_prompt(text: &str) {
    // 0xb8000 주소를 하드웨어 VGA 텍스트 버퍼 주소로 캐스팅하여 정적 참조 획득
    let writer = unsafe {
        &mut Writer {
            column_position: 0,
            // 검은색(Black) 배경에 밝은 회색(LightGray) 글자색으로 DOS 감성 지정
            color_code: ColorCode::new(Color::LightGray, Color::Black),
            buffer: &mut *(0xb8000 as *mut Buffer),
        }
    };
    writer.write_string(text);
}

/// 단일 문자 출력용 도우미 함수 (패닉 핸들러 등에서 긴급 출력용으로 사용)
#[allow(dead_code)]
pub unsafe fn print_char(byte: u8) {
    let writer = &mut *(0xb8000 as *mut Writer); // 최소한의 구조로 라이팅 시도
    writer.write_byte(byte);
}

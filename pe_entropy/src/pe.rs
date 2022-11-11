use crate::util::strip_0;

pub struct PEHeader<'a> {
    buf: &'a [u8],
}

impl<'a> PEHeader<'a> {
    // Offset to PE signature ("PE\0\0")
    fn pe_offset(&self) -> u32 {
        u32::from_le_bytes(self.buf[0x3c..0x40].try_into().unwrap())
    }

    fn number_of_sections(&self) -> u16 {
        let off = self.pe_offset() as usize + 0x6;
        u16::from_le_bytes(self.buf[off..(off + 2)].try_into().unwrap())
    }

    // relative to "MZ", not "PE"
    fn section_table_offset(&self) -> u32 {
        self.pe_offset() + 0xf8
    }
}
pub struct PE<'a> {
    header: PEHeader<'a>,
    buf: &'a [u8],
}

impl<'a> PE<'a> {
    /// constructor
    pub fn from_buf(buf: &'a [u8]) -> Self {
        PE {
            header: PEHeader { buf },
            buf,
        }
    }
    /// Get sections
    pub fn sections(&self) -> impl Iterator<Item = Section> {
        let off = self.header.section_table_offset() as usize;
        let count = self.header.number_of_sections() as usize;
        self.buf[off..] // 从PE文件中节表的开头开始
            .chunks(0x28) // 将buf划分成大小为0x28的块
            .map(|chunk| {
                // 将每个块解析成Section结构体

                let h = SectionHeader::from_buf(chunk.try_into().unwrap());
                let ptr = h.pointer_to_raw_data() as usize;
                let size = h.size_of_raw_data() as usize;
                Section {
                    header: h,
                    data: &self.buf[ptr..(ptr + size)],
                }
            })
            .take(count) // 有多少个节就解析多少个块
    }
}

pub struct Section<'a> {
    pub header: SectionHeader<'a>,
    pub data: &'a [u8], // Raw data of this section
}

pub struct SectionHeader<'a> {
    buf: &'a [u8; 0x28],
}

impl<'a> SectionHeader<'a> {
    pub fn from_buf(buf: &'a [u8; 0x28]) -> Self {
        Self { buf }
    }

    /// Name of a section without trailling '\0'
    pub fn name(&self) -> &[u8] {
        strip_0(self.name_raw())
    }

    pub fn name_raw(&self) -> &[u8; 8] {
        self.buf[0..8].try_into().unwrap()
    }

    pub fn size_of_raw_data(&self) -> u32 {
        u32::from_le_bytes(self.buf[0x10..0x14].try_into().unwrap())
    }

    pub fn pointer_to_raw_data(&self) -> u32 {
        u32::from_le_bytes(self.buf[0x14..0x18].try_into().unwrap())
    }
}

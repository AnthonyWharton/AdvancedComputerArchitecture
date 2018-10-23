// Example snippet of how to write Word to Memory
{
    use byteorder::{LittleEndian, WriteBytesExt};
    let word_vec: Vec<Word> = vec![0xaabbccdd, 2];
    let mut mem: Memory = vec![];

    for elem in word_vec {
        mem.write_u32::<LittleEndian>(elem).unwrap();
    }

    println!("\n\n TEST1 {:x?}", mem);
}

// Example snippet of how to read Word from Memory
{
    use byteorder::{LittleEndian, ReadBytesExt};
    
    let mem: Memory = Box::new(vec![0xaa, 0xbb, 0xcc, 0xdd, 0, 0, 0, 2]);
    let mut word_vec: Vec<Word> = vec!(0; 4);
   
    // Obviously this is for reading multiple lines, only need the read_u32 line
    let mut rdr = &mem[..];
    let mut idx = 0;
    for _ in mem.iter().step_by(4) {
        word_vec[idx] = rdr.read_u32::<LittleEndian>().unwrap();
        idx += 1;
    }
    
    println!("\n\n TEST1 {:x?}", word_vec);
}

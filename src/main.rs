struct hinhChuNhat {
    dai: u32, 
    rong: u32,
}

fn main() {
    let kichThuoc = hinhChuNhat {
        dai: 50,
        rong: 30,
    };

    println!("dien tich hinh chu nhat la {}", dien_tich(&kichThuoc))
}

fn dien_tich(kich_thuoc: &hinhChuNhat) -> u32 {
    kich_thuoc.dai * kich_thuoc.rong
}
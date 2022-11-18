
pub fn demo_forloop() {
    for x in 1..11
    // 1 to 10; 11..1 won't work
    {
        // skip 3
        if x == 3 {
            continue;
        }

        // stop at 7
        if x == 8 {
            break;
        }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn demo_whileloop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);
        // stop at 2^10
        if y == 1 << 10 {
            break;
        }
    }
}

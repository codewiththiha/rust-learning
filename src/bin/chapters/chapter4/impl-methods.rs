struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn can_fit(&mut self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn destory(self) {
        // not destory actually just moved and take usability actualy
        self.width;
        self.height;
    }

    fn move_var(self) -> Rectangle {
        Rectangle { ..self }
    }
}

fn main() {
    let mut new_r = Rectangle {
        width: 20,
        height: 40,
    };

    // new_r.destory(); // the rest won't work if we do this since ownership moved

    new_r.resize(50, 80);
    let area = new_r.area();
    println!("area: {}", area);
    let r_2 = Rectangle {
        width: 30,
        height: 70,
    };
    let r_3 = Rectangle {
        width: 80,
        height: 90,
    };
    let c1 = new_r.can_fit(r_2);
    let c2 = new_r.can_fit(r_3);
    println!("cases : {} , {}", c1, c2);
    let m_o = new_r.move_var();
    println!("value inside new owner{}", m_o.area());
    m_o.destory();
} // pretty clean damnnnn

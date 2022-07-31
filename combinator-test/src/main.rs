
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "John 3.5",
        "Jane 3.2",
        "Mary 3.7",
    ];

    let good_students: Vec<Student> = students.iter()
        .map(|s|{
            let mut s = s.split_whitespace();
            let name = s.next()?.to_owned();
            let gpa = s.next().unwrap().parse::<f32>().ok()?;
            Some(Student { name, gpa})
        })
        .flatten()
        .filter(|s|s.gpa >= 3.5)
        .collect();


        for s in good_students {
            println!("{}, {}", s.name, s.gpa);
        }

    //----------------------------

    let mut good_students = vec![];    

    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if name.is_some() && gpa.is_some(){
            let name = name.unwrap().to_owned();
            let gpa = gpa.unwrap().to_owned();

            let gpa = gpa.parse::<f32>();

            if gpa.is_ok(){
                let gpa = gpa.unwrap();
                if gpa >= 3.5 {
                    good_students.push(Student {
                        name: name,
                        gpa: gpa,
                    });
                }
            }
        }
    }

    for s in good_students {
        println!("{}, {}", s.name, s.gpa);
    }

}

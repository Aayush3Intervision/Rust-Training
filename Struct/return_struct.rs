
fn main(){
    let emp1 = Employee{
        emp_name: String::from("Ajay"),
        emp_id: 02,
        salary: 2333.0 
    };
    let emp2 = Employee{
        emp_name: String::from("Aayush"),
        emp_id: 01,
        salary: 2338.0 
    };

    let result = compair(emp1, emp2);
    println!("elder is");

    display(result);
}

fn display(emp: Employee){
    println!("Employee name, id and salary is as follows: {}, {}, {}",emp.emp_name,emp.emp_id,emp.salary);
}

fn compair(emp1: Employee, emp2: Employee) -> Employee{
    if emp1.salary > emp2.salary{
        return emp1;
    }
    else{
        return emp2;
    }
}
struct Employee{
    emp_name: String,
    emp_id: i32,
    salary:f64,
}
struct Employee{
    emp_name: String,
    emp_id: i32,
    salary:f64,
}

fn main(){
    let emp = Employee{
        emp_name: String::from("Ajay"),
        emp_id: 02,
        salary: 2333.0 
    };

    display(emp);
}

fn display(emp: Employee){
    println!("Employee name, id and salary is as follows: {}, {}, {}",emp.emp_name,emp.emp_id,emp.salary);
}
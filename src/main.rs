// Rust命令行简易待办清单 - 华为多模态实习岗应急项目
// 无第三方依赖，覆盖Rust核心基础语法，直接运行即可

fn main() {
    // 定义Vec集合存储待办项，String类型，初始为空（Rust的动态数组，核心集合类型）
    let mut todo_list: Vec<String> = Vec::new();
    // 欢迎语
    println!("===== Rust简易待办清单（To-Do List）=====");

    // 无限循环，实现交互菜单的持续运行，输入4则退出
    loop {
        // 打印操作菜单
        println!("请选择操作：");
        println!("1. 添加待办项");
        println!("2. 查看所有待办项");
        println!("3. 删除待办项");
        println!("4. 退出程序");
        print!("请输入序号（1-4）：");

        // 读取用户输入的命令（处理命令行输入，Rust基础IO操作）
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败"); // 简单错误处理，符合Rust的错误处理思想
        // 去除输入中的空格和换行符（用户输入可能带回车/空格，处理后更规范）
        let choice = input.trim();

        // 模式匹配（match）：Rust核心特性，替代其他语言的switch，处理菜单选择
        match choice {
            "1" => add_todo(&mut todo_list),  // 传递todo_list的可变引用（Rust的所有权/引用特性，核心考点）
            "2" => show_todo(&todo_list),     // 传递不可变引用，仅查看不修改
            "3" => delete_todo(&mut todo_list),// 传递可变引用，需要修改集合
            "4" => {
                // 退出程序
                println!("\n感谢使用，程序退出！");
                break; // 跳出无限循环，程序结束
            }
            // 处理无效输入，容错处理，避免程序崩溃
            _ => println!("\n❌ 输入错误！请输入1-4之间的序号\n"),
        }
    }
}

// 定义添加待办项的函数，参数是todo_list的可变引用（&mut Vec<String>）
fn add_todo(todo_list: &mut Vec<String>) {
    print!("\n请输入待办项内容：");
    let mut new_todo = String::new();
    std::io::stdin()
        .read_line(&mut new_todo)
        .expect("读取待办内容失败");
    let new_todo = new_todo.trim().to_string(); // 处理输入，转为String类型

    // 避免添加空内容
    if new_todo.is_empty() {
        println!("❌ 待办项内容不能为空！\n");
        return;
    }

    // 添加到待办列表
    todo_list.push(new_todo.clone());
    println!("✅ 待办项添加成功：{}\n", new_todo);
}

// 定义查看待办项的函数，参数是todo_list的不可变引用（&Vec<String>）
fn show_todo(todo_list: &Vec<String>) {
    println!("\n===== 我的待办清单 =====");
    // 判断待办列表是否为空
    if todo_list.is_empty() {
        println!("暂无待办项，快来添加吧！");
    } else {
        // 遍历待办列表，带序号输出（iter()迭代器，Rust的遍历方式）
        for (index, todo) in todo_list.iter().enumerate() {
            // index从0开始，加1转为用户习惯的1开始序号
            println!("{}. {}", index + 1, todo);
        }
    }
    println!("====================\n");
}

// 定义删除待办项的函数，参数是todo_list的可变引用（&mut Vec<String>）
fn delete_todo(todo_list: &mut Vec<String>) {
    // 如果待办列表为空，直接返回
    if todo_list.is_empty() {
        println!("❌ 暂无待办项，无需删除！\n");
        return;
    }

    // 先打印当前待办项，方便用户选择删除序号
    show_todo(todo_list);
    print!("请输入要删除的待办项序号：");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("读取序号失败");
    let input = input.trim();

    // 将输入的字符串转为数字（u32），并做容错处理（输入非数字时提示错误）
    match input.parse::<usize>() {
        Ok(index) => {
            // 序号转换：用户输入的1开始 → 数组的0开始
            let pos = index - 1;
            // 判断序号是否有效
            if pos < todo_list.len() {
                let deleted = todo_list.remove(pos); // 删除指定位置的元素，返回被删除的内容
                println!("✅ 成功删除待办项：{}\n", deleted);
            } else {
                println!("❌ 序号不存在！请输入有效序号\n");
            }
        }
        Err(_) => {
            // 转换失败（输入非数字），提示错误
            println!("❌ 输入错误！请输入数字序号\n");
        }
    }
}
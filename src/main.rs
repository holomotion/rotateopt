use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug)]
struct RotationMatrix {
    name: String,
    matrix: String,
}

const RULE_PATH:&str ="/etc/udev/rules.d/98-touchscreen-rotate.rules";

fn main() -> io::Result<()> {
    let rotations = vec![
        RotationMatrix {
            name: String::from("默认方向 (无旋转)"),
            matrix: String::from("1 0 0 0 1 0 0 0 1"),
        },
        RotationMatrix {
            name: String::from("竖向朝左 (逆时针90度)"),
            matrix: String::from("0 -1 1 1 0 0 0 0 1"),
        },
        RotationMatrix {
            name: String::from("竖向朝右 (顺时针90度)"),
            matrix: String::from("0 1 0 -1 0 1 0 0 1"),
        },
        RotationMatrix {
            name: String::from("上下颠倒 (180度旋转)"),
            matrix: String::from("-1 0 1 0 -1 1 0 0 1"),
        },
    ];

    println!("触摸屏旋转设置工具");
    println!("请选择旋转方向：");

    for (i, rotation) in rotations.iter().enumerate() {
        println!("{}. {}", i + 1, rotation.name);
    }

    print!("请输入选项 (1-{}): ", rotations.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selection = input.trim().parse::<usize>().unwrap_or(0);
    if selection < 1 || selection > rotations.len() {
        println!("无效的选择！");
        return Ok(());
    }

    let selected_rotation = &rotations[selection - 1];

    // 创建 udev 规则
    let rule_content = format!(
        r#"ACTION=="add|change", KERNEL=="event[0-9]*", ENV{{ID_INPUT_TOUCHSCREEN}}=="1", ENV{{LIBINPUT_CALIBRATION_MATRIX}}="{}"#,
        selected_rotation.matrix
    );

    match fs::write(RULE_PATH, rule_content) {
        Ok(_) => {
            println!("规则文件已成功写入到 {}", RULE_PATH);

            // 重新加载 udev 规则
            match Command::new("sudo")
                .args(&["udevadm", "control", "--reload-rules"])
                .status()
            {
                Ok(_) => {
                    println!("udev 规则已重新加载")
                }
                Err(e) => println!("重新加载 udev 规则失败: {}", e),
            }
        }
        Err(e) => println!("写入规则文件失败: {}", e),
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_content_format() {
        let test_matrix = RotationMatrix {
            name: String::from("测试方向"),
            matrix: String::from("1 0 0 0 1 0 0 0 1"),
        };

        let rule_content = format!(
            r#"ACTION=="add|change", KERNEL=="event[0-9]*", ENV{{ID_INPUT_TOUCHSCREEN}}=="1", ENV{{LIBINPUT_CALIBRATION_MATRIX}}="{}""#,
            test_matrix.matrix
        );

        let expected = r#"ACTION=="add|change", KERNEL=="event[0-9]*", ENV{ID_INPUT_TOUCHSCREEN}=="1", ENV{LIBINPUT_CALIBRATION_MATRIX}="1 0 0 0 1 0 0 0 1""#;

        assert_eq!(rule_content, expected);
    }
}
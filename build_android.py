#!/usr/bin/env python3

import os
import subprocess
import sys
import shutil

def check_ndk_path():
    ndk_path = "D:/android-ndk"
    if not os.path.isdir(ndk_path):
        print("未找到NDK路径，请确保NDK安装在D:/android-ndk")
        sys.exit(1)
    print("NDK路径检查通过")

def add_android_target():
    print("添加Android 64位目标...")
    try:
        # 确保在项目根目录执行rustup命令
        project_root = os.path.dirname(os.path.abspath(__file__))
        subprocess.run(["rustup", "target", "add", "aarch64-linux-android"], check=True, cwd=project_root)
    except subprocess.CalledProcessError as e:
        print(f"添加Android目标失败: {e}")
        sys.exit(1)

def run_fmt_and_clippy():
    print("检查代码格式...")
    try:
        # 确保在项目根目录执行cargo命令
        project_root = os.path.dirname(os.path.abspath(__file__))
        
        # 先检查是否需要格式化
        fmt_check_result = subprocess.run(["cargo", "fmt", "--", "--check"], 
                                        cwd=project_root, 
                                        capture_output=True, 
                                        text=True)
        
        if fmt_check_result.returncode == 0:
            print("代码格式检查通过，无需格式化")
        else:
            print("检测到代码格式问题，正在格式化...")
            subprocess.run(["cargo", "fmt"], check=True, cwd=project_root)
            print("代码格式化完成")
        
        # 运行clippy检查
        print("运行 clippy 检查...")
        subprocess.run([
            "cargo",
            "clippy",
            "--target",
            "aarch64-linux-android",
            "--",
            "-D",
            "warnings",
        ], check=True, cwd=project_root)
        print("clippy 检查通过")
    except subprocess.CalledProcessError as e:
        print(f"代码格式或检查失败: {e}")
        sys.exit(1)

def build_android():
    print("构建Android 64位版本...")
    try:
        # 确保在项目根目录执行cargo命令
        project_root = os.path.dirname(os.path.abspath(__file__))
        subprocess.run(["cargo", "build", "--target", "aarch64-linux-android", "--release"], 
                      check=True, cwd=project_root)
    except subprocess.CalledProcessError as e:
        print(f"构建Android版本失败: {e}")
        sys.exit(1)

def copy_binary_to_output():
    print("将构建的二进制文件复制到output文件夹...")
    try:
        project_root = os.path.dirname(os.path.abspath(__file__))
        source_path = os.path.join(project_root, "target", "aarch64-linux-android", "release", "death_note")
        output_dir = os.path.join(project_root, "output")
        
        # 创建output目录（如果不存在）
        os.makedirs(output_dir, exist_ok=True)
        
        # 复制二进制文件
        shutil.copy2(source_path, output_dir)
        print("二进制文件已成功复制到output文件夹！")
    except Exception as e:
        print(f"复制二进制文件失败: {e}")
        sys.exit(1)

def main():
    print("Death Note 构建脚本 (仅64位)")

    check_ndk_path()
    add_android_target()
    run_fmt_and_clippy()
    build_android()
    copy_binary_to_output()
    print("构建完成！")

if __name__ == "__main__":
    main()
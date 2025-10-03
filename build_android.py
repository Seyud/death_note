#!/usr/bin/env python3

import os
import shutil
import subprocess
import sys

try:
    import tomllib  # Python 3.11+
except ModuleNotFoundError:  # pragma: no cover
    try:
        import tomli as tomllib  # type: ignore[assignment]
    except ModuleNotFoundError:  # pragma: no cover
        print("未找到 tomllib/tomli，请使用 Python 3.11+ 或运行 `pip install tomli`")
        sys.exit(1)

def load_build_config():
    project_root = os.path.dirname(os.path.abspath(__file__))
    config_path = os.path.join(project_root, "config", "build_android.toml")

    if not os.path.isfile(config_path):
        print(f"未找到配置文件: {config_path}")
        sys.exit(1)

    with open(config_path, "rb") as f:
        try:
            return tomllib.load(f)
        except (tomllib.TOMLDecodeError, ValueError) as exc:
            print(f"解析配置文件失败: {exc}")
            sys.exit(1)


def get_required_path(config: dict, key: str) -> str:
    paths_section = config.get("paths")
    if not isinstance(paths_section, dict):
        print("配置文件缺少 `paths` 配置段")
        sys.exit(1)

    value = paths_section.get(key)
    if not value:
        print(f"配置文件缺少 `paths.{key}` 项")
        sys.exit(1)
    if not isinstance(value, str):
        print(f"`paths.{key}` 必须为字符串")
        sys.exit(1)

    resolved = os.path.expandvars(os.path.expanduser(value))
    if not os.path.isabs(resolved):
        project_root = os.path.dirname(os.path.abspath(__file__))
        resolved = os.path.join(project_root, resolved)
    return os.path.normpath(resolved)


def check_ndk_path(config):
    ndk_path = get_required_path(config, "ndk")
    if not os.path.isdir(ndk_path):
        print(f"未找到NDK路径: {ndk_path}")
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

def compress_binary_with_upx(config):
    print("使用UPX压缩二进制文件...")
    project_root = os.path.dirname(os.path.abspath(__file__))
    upx_path = get_required_path(config, "upx")
    binary_path = os.path.join(project_root, "output", "death_note")

    if not os.path.isfile(upx_path):
        print(f"未找到UPX执行文件: {upx_path}")
        sys.exit(1)

    if not os.path.isfile(binary_path):
        print(f"未找到待压缩的二进制文件: {binary_path}")
        sys.exit(1)

    result = subprocess.run([
        upx_path,
        binary_path,
    ], capture_output=True, text=True)

    if result.stdout:
        print(result.stdout.strip())
    if result.stderr:
        print(result.stderr.strip())

    if result.returncode == 0:
        print("UPX压缩完成")
    elif "AlreadyPackedException" in result.stdout:
        print("二进制文件已被UPX压缩，跳过")
    else:
        print("UPX压缩失败")
        sys.exit(result.returncode)

def main():
    print("Death Note 构建脚本")

    config = load_build_config()

    check_ndk_path(config)
    add_android_target()
    run_fmt_and_clippy()
    build_android()
    copy_binary_to_output()
    compress_binary_with_upx(config)
    print("构建完成！")

if __name__ == "__main__":
    main()
import sys
import shutil
import os
import json

CURRENT_DIR = os.path.dirname(os.path.abspath(__file__))

def j(*args):
    return os.path.join(*args)

def bj(*args):
    return j(CURRENT_DIR, *args)

# 输入：（以下只要在参数里出现空格的，一律需要加单双引号！）
# --project_name <项目名字（以中短横线为分隔符，全小写）>
# --project_underline_name <项目名字（以下划线为分隔符，全小写）>
# --project_stylized_name <项目全称（填入自己项目的全称，随便填，可以有空格，可以大写，但是尽量不要有中文。。）>
# --project_version <项目版本号（以点为分隔符，由数字做分隔。一般是有 3 个字符。）>
# --project_author <项目作者（可以填入自己的英文名，以及自己的邮箱。）>
# --project_description <项目描述（可以填入项目的简要描述。尽量以英文写。）>
# --project_license <项目许可证（可以填入项目的许可证类型。建议填入“All Rights Reserved”）>
# --project_identifier <项目全局唯一标识（建议填入：“com.作者名.作品名”，其中作品名只能由 26 个英文字母并且全部小写组成！不能加短横线！）>
PATTERN_MAP: dict = {}

with open(bj('gen', 'bin', 'saved.json'), 'r', encoding='UTF-8') as f:
    PATTERN_MAP = json.loads(f.read())

REPLACEMENTS = {}
args = sys.argv[1:]
i = 0
while i < len(args):
    key = args[i]
    if key in PATTERN_MAP:
        if i + 1 >= len(args):
            print(f'错误：参数 {key} 缺少对应的值', file=sys.stderr)
            sys.exit(1)
        old = PATTERN_MAP[key]
        new = args[i + 1]
        REPLACEMENTS[old] = new
        i += 2
    else:
        print(f'未知参数：{key}', file=sys.stderr)
        print(f'支持的参数：{list(PATTERN_MAP.keys())}', file=sys.stderr)
        sys.exit(1)

missing = [arg for arg in PATTERN_MAP if PATTERN_MAP[arg] not in REPLACEMENTS]
if missing:
    print(f'错误，缺少必要的参数：[{', '.join(missing)}]', file=sys.stderr)
    sys.exit(1)

RAW_UNDERLINE_NAME = PATTERN_MAP['--project_name'].replace('-', '_')
RAW_DIR_NAME = PATTERN_MAP['--project_identifier'].replace('.', '/')
RAW_UNDERLINE_IDENTIFIER = '_'.join(PATTERN_MAP['--project_identifier'].split('.'))
RAW_UNDERLINE_IDENTIFIER_BY_COMMA = ','.join(['_'.join(PATTERN_MAP['--project_identifier'].split('.')[:-1]), PATTERN_MAP['--project_identifier'].split('.')[-1]])
NEW_UNDERLINE_NAME = REPLACEMENTS[PATTERN_MAP['--project_name']].replace('-', '_')
NEW_DIR_NAME = REPLACEMENTS[PATTERN_MAP['--project_identifier']].replace('.', '/')
NEW_UNDERLINE_IDENTIFIER = '_'.join(REPLACEMENTS[PATTERN_MAP['--project_identifier']].split('.'))
NEW_UNDERLINE_IDENTIFIER_FRONT_TWO = '_'.join(REPLACEMENTS[PATTERN_MAP['--project_identifier']].split('.')[:-1])
NEW_UNDERLINE_IDENTIFIER_BY_COMMA = ','.join(['_'.join(REPLACEMENTS[PATTERN_MAP['--project_identifier']].split('.')[:-1]), REPLACEMENTS[PATTERN_MAP['--project_identifier']].split('.')[-1]])
ANDROID_PROJECT_DIR = j(CURRENT_DIR, 'gen', 'android')
KOTLIN_MAIN_PATH = j(ANDROID_PROJECT_DIR, 'app', 'src', 'main', 'kotlin')
MAIN_ACTIVITY_PATH = j(KOTLIN_MAIN_PATH, NEW_DIR_NAME)
if os.path.exists(KOTLIN_MAIN_PATH):
    shutil.rmtree(KOTLIN_MAIN_PATH)
os.makedirs(MAIN_ACTIVITY_PATH)
with open(j(MAIN_ACTIVITY_PATH, 'MainActivity.kt'), 'w', encoding='UTF-8') as f:
    f.write(f'package {REPLACEMENTS[PATTERN_MAP['--project_identifier']]}\n\nimport android.os.Bundle\n\n')
    f.write('''class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        initSysdirs(filesDir.absolutePath)
    }
    external fun initSysdirs(filesDir: String)
}''')

REPLACEMENTS[RAW_UNDERLINE_NAME] = NEW_UNDERLINE_NAME
REPLACEMENTS[RAW_DIR_NAME] = NEW_DIR_NAME
REPLACEMENTS[RAW_UNDERLINE_IDENTIFIER] = NEW_UNDERLINE_IDENTIFIER
REPLACEMENTS[RAW_UNDERLINE_IDENTIFIER_FRONT_TWO] = NEW_UNDERLINE_IDENTIFIER_FRONT_TWO
REPLACEMENTS[RAW_UNDERLINE_IDENTIFIER_BY_COMMA] = NEW_UNDERLINE_IDENTIFIER_BY_COMMA

EXCLUDED_PATHS = {
    bj('change_by_ci.py'),
    bj('change_by_json.py'),
    bj('ren-rs.config.json'),
    bj('target'),
    bj('README.md'),
    bj('LICENSE'),
    bj('.git'),
    bj('CROSS_PLATFORM.md')
}

def is_excluded(abs_path):
    for p in EXCLUDED_PATHS:
        if abs_path == p or abs_path.startswith(p):
            return True
    return False

def apply_replacements(text):
    for old, new in REPLACEMENTS.items():
        text = text.replace(old, new)
    return text


def main():
    all_files = []          # 所有需修改内容的文件（绝对路径）
    rename_items = []       # 需要重命名的项目 (旧路径, 新路径, 深度)
    for root, dirs, files in os.walk(CURRENT_DIR):
        if is_excluded(root):
            continue

        for f in files:
            full_old = j(root, f)
            if is_excluded(full_old):
                continue
            all_files.append(full_old)
            new_name = apply_replacements(f)
            if new_name != f:
                full_new = j(root, new_name)
                rename_items.append((full_old, full_new, full_old.count(os.sep)))
        for d in dirs:
            full_old = j(root, d)
            if is_excluded(full_old):
                continue
            new_name = apply_replacements(d)
            if new_name != d:
                full_new = j(root, new_name)
                rename_items.append((full_old, full_new, full_old.count(os.sep)))

    for file_path in all_files:
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            new_content = apply_replacements(content)
            if new_content != content:
                with open(file_path, 'w', encoding='utf-8', errors='ignore') as f:
                    f.write(new_content)
                print(f"📝 已修改内容：{file_path}")
        except Exception as e:
            print(f"❌ 修改内容失败 {file_path}：{e}")

    rename_items.sort(key=lambda x: x[2], reverse=True)
    for old, new, _ in rename_items:
        if os.path.exists(new):
            print(f"⚠️ 目标已存在：{old} -> {new}")
            continue
        try:
            os.rename(old, new)
            print(f"✅ 重命名：{old} -> {new}")
        except Exception as e:
            print(f"❌ 重命名失败 {old} -> {new}：{e}")

main()

import os
import sys
from shutil import copyfile

def run(cmd):
    print("Running command: " + cmd)
    result = os.system(cmd)
    if result != 0:
        raise RuntimeError("Command " + cmd + " failed")

def copy_file(path_from, path_to):
    print("Copying " + path_from + " to " + path_to)

    try:
        os.makedirs(os.path.dirname(os.path.abspath(path_to)), exist_ok=True)
        copyfile(path_from, path_to)
    except OSError as e:
        raise RuntimeError("Failed to copy from " + path_from + " to " + path_to)

def build_app(app_name):
    snake_case_name = app_name.replace("-", "_")
    run("wasm-pack build --target=web --no-typescript --release ./apps/{}".format(app_name))
    copy_file(
        "./apps/{0}/pkg/web_{1}_bg.wasm".format(app_name, snake_case_name),
        "./static/apps/web_{1}_bg.wasm".format(app_name, snake_case_name))
    copy_file(
        "./apps/{0}/pkg/web_{1}.js".format(app_name, snake_case_name),
        "./static/apps/web_{1}.js".format(app_name, snake_case_name))

WASM_APPS = ["freepdk-gen"]

def zola_build():
    run("zola build")

def main():
    try:
        for app_name in WASM_APPS:
            build_app(app_name)

        zola_build()


    except RuntimeError as e:
        print("Build failed: " + str(e))
        sys.exit(1)


if __name__ == '__main__':
    main()
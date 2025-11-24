import typer
import subprocess
import os
import string

app = typer.Typer()

def get_root_directory() -> str:
    base_path = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True).stdout
    return base_path.decode("utf-8")[0:-1]

def extract_route(route: str) -> str:
    if route.startswith("<"):
        return "index"
    return route.split(".")[0]

def get_template(template_name: str) -> string.Template:
    with open(os.path.join(get_root_directory(), "tools", "templates", f"{template_name}.template"), "r") as file:
        template = string.Template(file.read())
        return template

def generate_route(route_path: str, route: str) -> str:
    template = get_template("route")
    return template.safe_substitute(route_path=route_path, route=route)

def generate_mod_entry(file) -> str:
    template = get_template("mod_entry")
    return template.safe_substitute(file=extract_route(file))

def generate_mod(routes) -> str:
    template = get_template("mod")
    routes = ", ".join([f"{route}::{route}" for route in routes])
    return template.safe_substitute(routes=routes)

@app.command()
def generate_routes(view_path: str = None, generated_path: str = None):
    root_directory = get_root_directory()

    view_path = view_path or os.path.join(root_directory, "views")
    generated_path = generated_path or os.path.join(root_directory, "src", "routes")

    routes = []
    to_visit = [""]

    while to_visit:
        route = to_visit.pop(0)
        current_path = os.path.join(view_path, route)
        for item in sorted(os.listdir(current_path)):
            # todo: add support for dynamic routes.
            if os.path.isfile(os.path.join(current_path, item)):
                routes.append((route, extract_route(item)))
            else:
                to_visit.append(item)

    if not routes:
        return

    if not os.path.exists(generated_path):
        os.makedirs(generated_path)

    mod_path = os.path.join(generated_path, "mod.rs")

    with open(mod_path, "w") as file:
        generated = generate_mod(routes=[route[1] for route in routes])
        file.write(generated)

    for (route, file) in routes:
        path = os.path.join(generated_path, route)

        if not os.path.exists(path):
            os.makedirs(path)

        path = os.path.join(path, f"{file}.rs")

        if os.path.exists(path):
            pass


        with open(path, "w") as f:
            generated = generate_route(os.path.join(route, file), file)
            f.write(generated)

        with open(mod_path, "a") as f:
            generated = generate_mod_entry(file)
            f.write(generated)


if __name__ == "__main__":
    app()
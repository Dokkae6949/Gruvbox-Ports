# Gruvbox Ports
A simple (mostly) static website showcasing various
gruvbox ports to different applications or services.

## Setup
The application requires a PostgreSQL database and a connection
string provided in the `.env` file. The environment file has a
template provided in `.env.example` which can be copied and changed
accordingly.

Once a database is set up migrations need to either be run via the
application or from the terminal using the `sqlx migrate run` command.

To start the application it is sufficient to run `cargo run` in a terminal.

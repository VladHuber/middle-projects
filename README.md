# middle-projects

Средний уровень Rust: тринадцать учебных заданий — независимых крейтов (`task11` … `task23`),
каждое в своей папке со своим `Cargo.toml`. Это продолжение [FirstSteps](../FirstSteps)
и [Mini-project](../Mini-project), собранное по той же схеме: все задания лежат в одном
родительском репозитории обычными каталогами (не сабмодулями), поэтому любой файл можно
открыть прямо на GitHub.

## Требования

- Rust **1.85+** (во всех крейтах `edition = "2024"`)
- Стабильный тулчейн: https://rustup.rs
- Интернет нужен только `task16` и `task17` (качают страницы/файлы)
- SQLite ставить не нужно: `task11` использует `rusqlite` с фичей `bundled`

## Как запускать

Каждое задание — отдельный проект, команды выполняются из его папки:

```bash
cd task11
cargo run -- list
```

## Задания

| # | Что делает | Как запускать | Ключевые приёмы | Зависимости |
|---|------------|---------------|-----------------|-------------|
| [task11](task11) | CLI-менеджер задач (todo) поверх SQLite: слои `main → service → repository → db`, таблица `tasks(id, title, is_done)` | `cargo run -- add "текст"`, `cargo run -- list`, `cargo run -- done <id>`, `cargo run -- delete <id>` | `rusqlite::Connection`, `params![]`, `query_map`, маппинг `bool` ↔ `INTEGER`, `Result<_, Box<dyn Error>>` | `rusqlite` 0.32 (bundled) |
| [task12](task12) | Синхронный загрузчик файла по URL: до 3 попыток подключения, таймауты, чтение блоками по 8192 байта и свой прогресс-бар (%, МБ, МБ/с, ETA) | `cargo run -- download <URL>` | `reqwest::blocking::Client` с `connect_timeout`/`timeout`, `url::Url`, retry-цикл, `File::create` + `read`/`write_all`, `Instant::elapsed`, `flush()` | `reqwest` 0.12 (blocking), `url` 2.5.8 |
| [task13](task13) | CLI-архиватор zip: создать архив из папки, показать содержимое, распаковать | `cargo run -- archive create <zip> <папка>`, `... archive list <zip>`, `... archive extract <zip> <папка>` | `walkdir::WalkDir`, `zip::{ZipWriter, ZipArchive, SimpleFileOptions}`, `strip_prefix`, замена `\` на `/` в именах, `enclosed_name()` (защита от zip-slip) | `walkdir` 2.5.0, `zip` 8.6.0 |
| [task14](task14) | CLI-редактор TOML-конфига: показать конфиг и изменить одно поле по ключу, с валидацией порта и ошибкой на неизвестный ключ | `cargo run -- show`; `cargo run -- set <key> <value>` (`app_name`, `port`, `debug`, `database.host`, `logging.level`, …) | `serde` derive, вложенные структуры, `toml::from_str` / `to_string_pretty`, `match` по строковому ключу | `serde` 1.0.228 (derive), `toml` 1.1.2 |
| [task15](task15) | Собственный пул потоков: `ThreadPool::new(4)` и 100 задач, корректное завершение воркеров | `cargo run` | `mpsc::channel`, `Arc<Mutex<Receiver<Job>>>`, `type Job = Box<dyn FnOnce() + Send + 'static>`, `impl Drop` с `join` | — |
| [task16](task16) | Асинхронный «плоский» краулер: обходит сайт, собирает ссылки `a[href]`, фильтрует по домену и `.exe`, ограничивает параллелизм семафором | `cargo run` (стартовый URL `https://stamina.ru` и `max_depth = 3` зашиты в код) | `#[tokio::main]`, `tokio::spawn`, `tokio::sync::{mpsc, Mutex, Semaphore}`, `acquire_owned`, `scraper::{Html, Selector}`, `HashSet` для посещённых | `tokio` 1 (full), `reqwest` 0.12, `scraper` 0.20, `url` 2 |
| [task17](task17) | Менеджер загрузок: читает список URL из файла и качает их параллельно (не более 5), рисуя прогресс-бар; при совпадении имён делает `имя (1).ext` | `cargo run -- urls.txt` (файл ищется в `assets/`) | `tokio::spawn` + `Arc<Semaphore>`, `tokio::fs` + `AsyncBufReadExt`, `bytes_stream()` + `StreamExt::next()`, `indicatif::{ProgressBar, ProgressStyle}` | `reqwest` 0.13.4 (stream), `tokio` 1.53.1, `indicatif` 0.17, `futures-util` 0.3, `url` 2.5.8 |
| [task18](task18) | Минимальный HTTP-сервер на axum: `GET /` → `Hello, World!`, `POST /users`, `DELETE /users` | `cargo run` (слушает `127.0.0.1:3000`), проверяется curl | `axum::{Router, routing::{get, post, delete}, Json}`, async-хендлеры, `tokio::net::TcpListener::bind`, `axum::serve` | `axum` 0.7, `tokio` 1 (full), `serde` 1 (derive) |
| [task19](task19) | REST-сокращатель ссылок: `POST /shorten` → 201 и короткий id, `GET /{id}` → 307 redirect или 404 | `cargo run` (порт 3000): `curl -X POST http://127.0.0.1:3000/shorten -d '{"url":"..."}'` | `Router::with_state`, экстракторы `State`/`Path`/`Json`, `Repository` с `Arc<Mutex<HashMap>>`, `rand` Alphanumeric-идентификатор из 5 символов, `Redirect::temporary`, `StatusCode` | `axum` 0.8, `tokio` 1 (full), `serde` 1 (derive), `rand` 0.8 |
| [task20](task20) | Pastebin в памяти: `POST /paste` → 201 и id, `GET /{id}` → текст или 404; в `helper.txt` лежат готовые curl-команды | `cargo run` (порт 3000) | То же, что в task19: `Arc<Mutex<HashMap>>`, `Repository::save`/`find`, случайный id, `StatusCode::NOT_FOUND` | `axum` 0.8, `tokio` 1 (full), `serde` 1 (derive), `rand` 0.8 |
| [task21](task21) | Учебный аналог git: `init`, `add`, `status`, `commit`, `log`; объекты по SHA-1 в `.mygit/objects/<2>/<38>`, дерево через bincode, история по `parent` | `cargo run -- init`, `cargo run -- add <файл>`, `cargo run -- status`, `cargo run -- commit "..."`, `cargo run -- log` | `sha1::{Sha1, Digest}`, `hex::encode`, `bincode::serialize`, `BufReader::lines` + `split_once`, рекурсивный обход `parent` | `sha1` 0.10, `hex` 0.4, `bincode` 1.3, `serde` 1.0 (derive) |
| [task22](task22) | Учебный менеджер пакетов (мини-cargo): создаёт проект `my_project`, читает манифест, ищет версии в registry, ставит максимальную подходящую по semver, удаляет и обновляет | `cargo run -- init`, `cargo run -- read`, `cargo run -- search <пакет>`, `cargo run -- install <пакет>`, `cargo run -- remove <пакет>`, `cargo run -- update` | `#[serde(untagged)] enum Dependency`, `BTreeMap`, `toml::from_str`/`to_string_pretty`, `semver::{Version, VersionReq}` (`matches`, `max`), `fs_extra::dir::{copy, remove}` | `toml` 0.9, `serde` 1.0.28 (derive), `semver` 1.0.28, `fs_extra` 1.3 |
| [task23](task23) | Три бинарника: TCP-сервер на 6379, отвечающий `HTTP/1.1 200 OK`; key-value сервер на 8080 (`SET`/`GET`/`DEL`); интерактивный клиент к нему | `cargo run` (6379), `cargo run --bin server` (8080), `cargo run --bin client` | `std::net::{TcpListener, TcpStream}`, `thread::spawn`, `Arc<Mutex<HashMap>>`, сборка частичных TCP-сообщений через `pending` + `find('\n')` + `splitn(3, " ")` | — |

## Данные для запуска

Часть заданий работает с файлами. Что уже есть в репозитории, а что создаётся или скачивается при запуске:

- **task11** — `mydb.db` создаётся сам при первом запуске (`CREATE TABLE IF NOT EXISTS tasks`), в git не попадает.
- **task12** — скачанные файлы складываются в `download/` (папка игнорируется); пример URL есть в комментарии `main.rs`.
- **task13** — для `archive create` нужна любая папка с файлами; тестовые архивы и распакованный `folder/` в репозиторий не попадают из-за размера.
- **task14** — `configs/config.toml` **в репозитории** (без него программа не запустится); `configs/parser.rs` — старая копия парсера, в сборку не входит.
- **task17** — список ссылок `assets/urls.txt` (20 URL) в репозитории, качает в `download/`.
- **task21** — служебный каталог `.mygit/` создаётся командой `init` и игнорируется.
- **task22** — проект `my_project/` (манифест, `src/main.rs`, `registry/`, `.mycargo/installed/`) создаётся командами и игнорируется.

## Специфичные файлы и `.gitignore`

Корневой `.gitignore` закрывает всё, что не должно попадать в репозиторий:

```
target/, **/target/     сборка каждого крейта
**/download/            скачанное (task12, task17)
*.zip *.7z *.rar ...    тестовые архивы (task13) и любые архивы исходников
*.exe *.msi *.dll       скачанные и распакованные бинарники
*.db *.sqlite*          база task11
task13/folder/          распакованное содержимое тестовых архивов
task21/.mygit/          тестовый репозиторий, создаваемый task21
task22/my_project/      проект, создаваемый task22
task11/refactor AI.md, **/refactor*.md, **/refactor*.odt   артефакты ИИ-ревью
```

Архивы с исходниками, которые отправлялись на проверку ИИ (`task19/src/handlers.zip`, `task14/configs/service.zip`), удалены — их содержимое и так лежит в `src/`.

## Структура репозитория

```
middle-projects/
├── README.md
├── .gitignore
├── task11/ … task23/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── .gitignore          (/target и локальные данные задачи)
│   ├── src/                (main.rs, модули; в task23 ещё src/bin/{server,client}.rs)
│   ├── assets/, configs/, doc/ — входные данные и документы задания
│   └── target/             (сборка, в git не попадает)
```

## Заметки по коду

- Задания учебные: местами остались `unwrap()`/`expect()`, закомментированный код и мёртвые ветки
  (`task13/src/service.rs`, `task23/src/main.rs`, пустой цикл в `task16`), поэтому компилятор может
  выдавать предупреждения.
- Неиспользуемые зависимости и фичи: `serde_json` объявлен в `task14`, но в коде не встречается;
  `reqwest` с фичей `json` в `task16` (используется только `.text()`); `axum` с фичей `macros`
  в `task19`/`task20`.
- Захардкоженные значения: `https://stamina.ru`, `max_depth = 3` и 5 воркеров в `task16`;
  порты `127.0.0.1:3000` в `task18`–`task20`, `6379` и `8080` в `task23`; `id = 1337` в `task18`.
- Хранилище `task19`/`task20` — в памяти, при перезапуске данные теряются.
- Мелкие слабые места: в `task13` режим `list` читает `args[3]`, поэтому вызов без имени файла
  паникует; в `task21` пустой `.mygit/refs/heads/master` приведёт к панике в `slice_hash`;
  в `task11` при вставке `Todo.id` всегда 0 (реальный id выдаёт AUTOINCREMENT).
- Каждая задача раньше была отдельным локальным git-репозиторием (без remote). История сохранена
  локально в `../middle-projects-submodules-backup/taskNN.git`, в этот репозиторий она не переносилась.

use clap::{crate_description, crate_version, Parser, Subcommand};

// pub const CONFIG_OPTIONS: [&str; 3] = ["depth", "height", "finder"];

#[derive(Debug, Parser)]
#[command(
    name = "repo",
    // after_help = ARG_AFTER_HELP_MSG,
    about = crate_description!(),
    version = crate_version!(),
    max_term_width = 100,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
    #[arg(long, default_value_t = false)]
    pub list_command_options: bool,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Add(AddCmd),
    Config(ConfigCmd),
    Edit(EditCmd),
    Foreach(ForeachCmd),
    Inspect(InspectCmd),
    Init(InitCmd),
    List(ListCmd),
    Remove(RemoveCmd),
    Tag(TagCmd),
    Update(UpdateCmd),
    Work(WorkCmd),
}

/// Add a repository to be tracked by repo
#[derive(Debug, Parser)]
#[command(
    name = "repo add",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct AddCmd {
    /// A url link to the repository's remote origin
    ///
    /// Url can be represented by the following specifications:
    ///   * <scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git
    ///     - Available schemes are: `http[s]`, `ssh` and `git`.
    ///     - Example: https://github.com/user/repo
    ///   * <username>@<host>:<path-to-repo>
    ///     - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
    ///     - Example: git@github.com:user/repo
    ///   * <path-to-repo>
    ///     - This option uses the config file to construct the url to the
    ///       remote repository.
    /// If url or scheme is not defined in the config file they will be
    /// defaulted to:
    ///   scheme:   'https'
    ///   host:     'github.com'
    ///   ssh_user: 'git'
    ///     - Example: rust-lang/cargo
    #[arg(verbatim_doc_comment)]
    pub url: String,

    /// Name of the repository
    #[arg(default_value = None)]
    pub name: Option<String>,

    /// Add a tag to repository
    ///
    /// The repository will inherit all properties from a tag. Tags have a
    /// priority and will be ordered by priority. The lowest priority will be
    /// evaluated first.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub tags: Option<Vec<String>>,

    /// Add an additional remote
    ///
    /// Remote will be appended to the repository as an additional remote This
    /// is useful if the repository is a fork, letting you link to the upstream
    /// remote.  Repo uses the first remote in it's list as the default remote.
    /// By convention the first remote is 'origin'. Remote's argument format is
    /// name and url separated by a ','
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub remotes: Option<Vec<String>>,

    /// Execute command after being cloned by the update command
    ///
    /// If this repository contains links to tags that also contain 'clone'
    /// actions the repository actions will be executed first followed by the
    /// tags, ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub clone: Option<String>,

    /// Execute command after calling the work command
    ///
    /// If this repository contains links to tags that also contain 'work'
    /// actions the repository actions will be executed first followed by the
    /// tags, ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub work: Option<String>,

    /// Override the default path of the repository in the workspace
    ///
    /// By default, the workspace path of a repository is based on the name of
    /// the repository.  This option will override this behaviour and set the
    /// workspace path.  If a repository also has a path definition it will
    /// override a tag's.
    ///
    /// Note: Relative paths are relative to the workspace root.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub path: Option<String>,

    /// Write repository to local cache
    ///
    /// Local cache is defined by '$REPO_LOCAL_PATH' environment variable.
    /// If env var is not set then repo will default to your system's local
    /// data folder:
    ///   - linux: $HOME/.local/share/repo
    ///   - windows: C:\Users\<USER>\AppData\Local\repo
    ///   - macos: /Users/<USER>/Library/Application Support/repo
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub local: bool,

    /// Override repository if it is already tracked by repo
    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    /// Flag repository to interact with git through the command line
    ///
    /// If for some reason git cannot access your remote repository you can
    /// specify a repository to use the command line instead of libgit2. This
    /// mainly happens because of authentication issues If you can get the
    /// command line to clone the repository the repo will use that instead.
    #[arg(short = 'u', long, verbatim_doc_comment, default_value_t = false)]
    pub cli: bool,
}

/// Get or set configuration options
#[derive(Debug, Parser)]
#[command(
    name = "repo config",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct ConfigCmd {
    /// Name of configuration option
    #[arg(index = 1, default_value = None)]
    pub name: Option<String>,

    /// Value to be set to the configuration option provided
    #[arg(index = 2, default_value = None)]
    pub value: Option<String>,

    /// Interact with local config.
    #[arg(short, long, conflicts_with = "global", default_value_t = false)]
    pub local: bool,

    /// Interact with global config.
    #[arg(short, long, conflicts_with = "local", default_value_t = false)]
    pub global: bool,

    /// Remove tag instead of adding
    ///
    /// Remove from 'include' or 'exclude' list
    #[arg(short, long = "rm", verbatim_doc_comment, default_value_t = false)]
    pub remove: bool,

    /// Open cache file in $EDITOR
    ///
    /// If $EDITOR is not defined will open in vim
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub edit: bool,

    /// List all config options and values
    #[arg(short = 's', long, default_value_t = false)]
    pub list: bool,

    /// List only config option names
    #[arg(short, long, default_value_t = false)]
    pub name_only: bool,
}

/// Edit a repository tracked by repo
#[derive(Debug, Parser)]
#[command(
    name = "repo edit",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct EditCmd {
    /// Name of the repository to be edited
    #[arg()]
    pub name: String,

    /// Override the default path of an attached repository in the workspace
    ///
    /// By default, the workspace path of a repository is based on the name of
    /// the repository.  This option will override this behaviour and set the
    /// workspace path.  If a repository also has a path definition it will
    /// override a tag's.
    ///
    /// Note: Relative paths are relative to the workspace root.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub path: Option<String>,

    /// Execute command after being cloned by the update command
    ///
    /// If this repository contains links to tags that also contain 'clone'
    /// actions the repository actions will be executed first followed by the
    /// tags, ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub clone: Option<String>,

    /// Execute command after calling the work command
    ///
    /// If this repository contains links to tags that also contain 'work'
    /// actions the repository actions will be executed first followed by the
    /// tags, ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub work: Option<String>,

    /// Add tag to repository
    ///
    /// The repository will inherit all properties from a tag. Tags have a
    /// priority and will be ordered by priority. The lowest priority will be
    /// evaluated first.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub tags: Option<Vec<String>>,

    /// Add an additional remote
    ///
    /// Remote will be appended to the repository as an additional remote This
    /// is useful if the repository is a fork, letting you link to the upstream
    /// remote.  Repo uses the first remote in it's list as the default remote.
    /// By convention the first remote is 'origin'. Remote's argument format is
    /// name and url separated by a ','
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub remotes: Option<Vec<String>>,

    /// Change repository to be a stored in the local cache
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Change repository to be a stored in the global cache
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Open cache file in $EDITOR
    #[arg(short, long, default_value_t = false)]
    pub edit: bool,

    /// Flag repository to interact with git through the command line
    ///
    /// If for some reason git cannot access your remote repository you can
    /// specify a repository to use the command line instead of libgit2. This
    /// mainly happens because of authentication issues If you can get the
    /// command line to clone the repository the repo will use that instead.
    #[arg(short = 'u', long, verbatim_doc_comment, default_value = None)]
    pub cli: bool,
}

/// Execute command for every tracked repository
#[derive(Debug, Parser)]
#[command(
    name = "repo foreach",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct ForeachCmd {
    /// Shell command to be executed
    #[arg()]
    pub cmd: String,

    /// Perform operation on only repositories that contain tag
    #[arg(short, long, default_value = None)]
    pub tags: Option<Vec<String>>,

    /// Perform operation on only local repositories
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Perform operation on only global repositories
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Perform operation on all repositories, global and local
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

/// Prints the shell function used to integrate repo with shell
#[derive(Debug, Parser)]
#[command(
    name = "repo init",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct InitCmd {
    /// Name of the shell the shell function will generate
    #[arg(value_parser = ["bash", "zsh", "fish"])] // TODO: possbile values
    pub shell: String,

    /// Intilaize with fzf integration
    #[arg(short, long, default_value_t = false)]
    pub fzf: bool,
}

/// Inspect a repository and view it's properties
#[derive(Debug, Parser)]
#[command(
    name = "repo inspect",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct InspectCmd {
    /// Name of the repository to be inspected
    #[arg()]
    pub name: String,

    /// Define the output format of the inspection
    #[arg(short, long)] // TODO: possible values
    pub format: Option<String>,
}

/// List repositories tracked by repo
#[derive(Debug, Parser)]
#[clap(name = "repo list")]
#[command(disable_colored_help(true), disable_version_flag(true))]
pub struct ListCmd {
    /// Show repositories that contain a tag
    #[arg(short, long, number_of_values = 1, default_value = None)]
    pub tags: Option<Vec<String>>,

    /// Show only local repositories
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Show only global repositories
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Show all repositories regardless of config filters
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

/// Remove a repository tracked by repo
#[derive(Debug, Parser)]
#[command(
    name = "repo remove",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct RemoveCmd {
    /// Name of repository
    #[arg(short, long)]
    pub names: Vec<String>,

    /// Force removal of repository without a conformation prompt.
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

/// Update tracked repositories in repo with their remotes
#[derive(Debug, Parser)]
#[command(
    name = "repo update",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct UpdateCmd {
    /// Perform operation on only local repositories
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Perform operation on only global repositories
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Perform operation on all repositories, global and local
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// Perform operation on only repositories that contain tag
    #[arg(short, long, default_value = None)]
    pub tags: Option<Vec<String>>,
}

/// Generate work command for a repostory
#[derive(Debug, Parser)]
#[command(
    name = "repo work",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct WorkCmd {
    /// Name of the tracked repository to be worked on
    #[arg()]
    pub name: String,

    /// Only change directory to repository in workspace
    ///
    /// This will not run the after 'work' post hook.
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub quick: bool,
}

/// Manage tags
#[derive(Debug, Parser)]
#[command(
    name = "repo tag",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct TagCmd {
    #[clap(subcommand)]
    pub cmd: TagSubCmd,
}

#[derive(Debug, Subcommand)]
pub enum TagSubCmd {
    Add(TagAddCmd),
    Edit(TagEditCmd),
    List(TagListCmd),
    Remove(TagRemoveCmd),
}

/// Add a tag to repo
#[derive(Debug, Parser)]
#[command(
    name = "repo tag add",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct TagAddCmd {
    /// Name of the tag
    pub name: String,

    /// Override the default path of an attached repository in the workspace.
    ///
    /// By default, the workspace path of a repository is based on the name of
    /// the repository.  This option will override this behaviour and set the
    /// workspace path.  If a repository also has a path definition it will
    /// override a tag's.
    ///
    /// Note: Relative paths are relative to the workspace root.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub path: Option<String>,

    /// Execute command after being cloned by the update command
    ///
    /// If a repository contains links to tags that also contain 'clone' actions
    /// the repository actions will be executed first followed by the tags,
    /// ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub clone: Option<String>,

    /// Execute command after calling the work command
    ///
    /// If a repository contains links to tags that also contain 'work' actions
    /// the repository actions will be executed first followed by the tags,
    /// ordered by priority
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub work: Option<String>,

    /// Set the tag priority. Tags will be applied from lowest to highest.
    /// Priority ties are resolved alphabetically
    #[arg(short = 'n', long, verbatim_doc_comment, default_value = None)]
    pub priority: Option<i32>,

    /// Write repository to local cache
    ///
    /// Local cache is defined by '$REPO_LOCAL_PATH' environment variable.
    /// If env var is not set then repo will default to your system's local
    /// data folder:
    ///   - linux: $HOME/.local/share/repo
    ///   - windows: C:\\Users\\<USER>\\AppData\\Local\\repo
    ///   - macos: /Users/<USER>/Library/Application Support/repo
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub local: bool,
}

/// Edit a tag stored in repo
#[derive(Debug, Parser)]
#[command(
    name = "repo tag edit",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct TagEditCmd {
    /// Name of the repository to be edited
    #[arg(default_value = None)]
    pub name: String,

    /// Override the default path of an attached repository in the workspace
    ///
    /// By default, the workspace path of a repository is based on the name of
    /// the repository.  This option will override this behaviour and set the
    /// workspace path.  If a repository also has a path definition it will
    /// override a tag's.
    ///
    /// Note: Relative paths are relative to the workspace root.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub path: Option<String>,

    /// Execute command after being cloned by the update command.
    ///
    /// If a repository contains links to tags that also contain 'clone' actions
    /// the repository actions will be executed first followed by the tags,
    /// ordered by priority.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub clone: Option<String>,

    /// Execute command after calling the work command.
    ///
    /// If a repository contains links to tags that also contain 'work' actions
    /// the repository actions will be executed first followed by the tags,
    /// ordered by priority.
    #[arg(short, long, verbatim_doc_comment, default_value = None)]
    pub work: Option<String>,

    /// Set the tag priority.
    ///
    /// Tags will be applied from lowest to highest. Priority ties
    /// are resolved alphabetically
    #[arg(short = 'n', long, default_value = None)]
    pub priority: Option<i32>,

    /// Change tag to be a stored in the local cache
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Change tag to be a stored in the global cache
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Open cache file in $EDITOR
    ///
    /// If $EDITOR is not defined will open in vim
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub edit: bool,
}

/// List tags stored in repo
#[derive(Debug, Parser)]
#[command(
    name = "repo tag list",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct TagListCmd {
    /// Show only local tags
    #[arg(short, long, default_value_t = false)]
    pub local: bool,

    /// Show only global tags
    #[arg(short, long, default_value_t = false)]
    pub global: bool,
}

/// Remove a tag from repo
#[derive(Debug, Parser)]
#[command(
    name = "repo tag remove",
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct TagRemoveCmd {
    /// Name of the tag to be removed from repo
    #[arg(help = "Name of tag")]
    pub names: Vec<String>,

    /// Force removal tag without a conformation prompt.
    #[arg(short, long, help = "Force removal of tag", default_value_t = false)]
    pub force: bool,
}

// vim: textwidth=80

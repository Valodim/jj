use jujutsu_lib::index::IndexEntry;
    short_commit_description, short_commit_hash, write_commit_summary, Args, CommandError,
    CommandHelper, WorkspaceCommandHelper,
    #[command(subcommand)]
    #[command(subcommand)]
    #[command(visible_alias = "op")]
    #[command(subcommand)]
    #[command(subcommand)]
    #[command(subcommand)]
#[command(group(ArgGroup::new("backend").args(&["git", "git_repo"])))]
    #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
    #[arg(long)]
    #[arg(long, value_hint = clap::ValueHint::DirPath)]
#[command(visible_aliases = &["co", "update", "up"])]
    #[arg(short = 'r', hide = true)]
    #[arg(long, short, default_value = "")]
    #[arg(required = true, value_hint = clap::ValueHint::AnyPath)]
    #[arg(long, short, default_value = "@")]
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    #[arg(long, short, default_value = "@")]
    #[arg(value_hint = clap::ValueHint::FilePath)]
#[command(group(ArgGroup::new("format").args(&["summary", "git", "color_words"])))]
    #[arg(long, short)]
    #[arg(long)]
    #[arg(long)]
    #[arg(long, short)]
    #[arg(long, conflicts_with = "revision")]
    #[arg(long, conflicts_with = "revision")]
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    #[command(flatten)]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
    #[command(flatten)]
#[command(visible_alias = "st")]
    /// Which revisions to show. Defaults to the `ui.default-revset` setting,
    /// or "remote_branches().. | (remote_branches()..)-" if it is not set.
    #[arg(long, short)]
    revisions: Option<String>,
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    #[arg(long)]
    #[arg(long)]
    #[arg(long, short = 'T')]
    #[arg(long, short = 'p')]
    #[command(flatten)]
    #[arg(long, short, default_value = "@")]
    #[arg(long)]
    #[arg(long, short = 'T')]
    #[arg(long, short = 'p')]
    #[command(flatten)]
#[command(group(ArgGroup::new("to_diff").args(&["from", "to"]).multiple(true).required(true)))]
    #[arg(long)]
    #[arg(long)]
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    #[command(flatten)]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
    #[arg(long, short)]
    #[arg(long)]
#[command(visible_alias = "commit", hide = true)]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
    #[arg(long, short)]
    #[arg(long, short)]
#[command(alias = "uncommit", hide = true)]
    #[arg(short = 'r', hide = true)]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
#[command(visible_alias = "hide")]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
    #[arg(short = 'r', hide = true)]
    #[arg(default_value = "@")]
    #[arg(short = 'r', hide = true)]
    #[arg(long, short, default_value = "")]
#[command(group(ArgGroup::new("to_move").args(&["from", "to"]).multiple(true).required(true)))]
    #[arg(long)]
    #[arg(long)]
    #[arg(long, short)]
    #[arg(conflicts_with = "interactive", value_hint = clap::ValueHint::AnyPath)]
#[command(visible_alias = "amend")]
    #[arg(long, short, default_value = "@")]
    #[arg(long, short)]
    #[arg(conflicts_with = "interactive", value_hint = clap::ValueHint::AnyPath)]
#[command(visible_alias = "unamend")]
    #[arg(long, short, default_value = "@")]
    #[arg(long, short)]
    #[arg(long)]
    #[arg(long)]
    #[arg(long, short)]
    #[arg(conflicts_with = "interactive", value_hint = clap::ValueHint::AnyPath)]
    #[arg(long, short, default_value = "@")]
    #[arg(long, short, default_value = "@")]
    #[arg(value_hint = clap::ValueHint::AnyPath)]
#[command(verbatim_doc_comment)]
#[command(group(ArgGroup::new("to_rebase").args(&["branch", "source", "revision"])))]
    #[arg(long, short)]
    #[arg(long, short)]
    #[arg(long, short)]
    #[arg(long, short, required = true)]
    #[arg(long, short, default_value = "@")]
    #[arg(long, short, default_value = "@")]
    #[command(visible_alias("c"))]
        #[arg(long, short)]
        #[arg(required = true)]
    #[command(visible_alias("d"))]
        #[arg(required = true)]
    #[command(visible_alias("f"))]
        #[arg(required = true)]
    #[command(visible_alias("l"))]
    #[command(visible_alias("s"))]
        #[arg(long, short)]
        #[arg(long)]
        #[arg(required = true)]
    #[arg(default_value = "@")]
    #[arg(long)]
    #[arg(long, value_hint = clap::ValueHint::AnyPath)]
    #[arg(long, conflicts_with = "clear", value_hint = clap::ValueHint::AnyPath)]
    #[arg(long)]
    #[arg(long, conflicts_with_all = &["add", "remove", "clear"])]
    #[arg(long, conflicts_with_all = &["add", "remove", "clear", "reset"])]
    #[command(subcommand)]
    #[arg(long, default_value = "origin")]
    #[arg(value_hint = clap::ValueHint::DirPath)]
    #[arg(value_hint = clap::ValueHint::DirPath)]
#[command(group(ArgGroup::new("what").args(&["branch", "all", "change"])))]
    #[arg(long, default_value = "origin")]
    #[arg(long)]
    #[arg(long)]
    #[arg(long)]
    #[arg(long)]
#[command(hide = true)]
    #[command(name = "resolverev")]
    #[command(name = "workingcopy")]
    #[command(name = "reindex")]
    #[arg(long, verbatim_doc_comment)]
    #[arg(long, verbatim_doc_comment)]
    #[arg(long, verbatim_doc_comment)]
    #[arg(long, short, default_value = "@")]
    #[arg(default_value = "@")]
        formatter.add_label("removed")?;
        formatter.add_label("added")?;
                    formatter.add_label("removed")?;
                    formatter.add_label("added")?;
    formatter.add_label("diff")?;
                formatter.add_label("header")?;
                formatter.add_label("header")?;
                formatter.add_label("header")?;
        formatter.add_label("hunk_header")?;
                    formatter.add_label("context")?;
                    formatter.add_label("removed")?;
                    formatter.add_label("added")?;
    formatter.add_label("diff")?;
        formatter.add_label("file_header")?;
    formatter.add_label("diff")?;
                formatter.add_label("modified")?;
                formatter.add_label("added")?;
                formatter.add_label("removed")?;
    let mut formatter = ui.stdout_formatter();
    let formatter = formatter.as_mut();
    if let Some(wc_commit) = &maybe_checkout {
        formatter.write_str("Parent commit: ")?;
        write_commit_summary(
            formatter,
            &wc_commit.parents()[0],
            ui.settings(),
        formatter.write_str("\n")?;
        formatter.write_str("Working copy : ")?;
        write_commit_summary(
            formatter,
            repo.as_repo_ref(),
            &workspace_id,
            wc_commit,
            ui.settings(),
        )?;
        formatter.write_str("\n")?;
        formatter.write_str("No working copy\n")?;
        formatter.add_label("conflict")?;
        writeln!(formatter, "These branches have conflicts:")?;
        formatter.remove_label()?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "{}", branch_name)?;
            formatter.remove_label()?;
            writeln!(formatter)?;
            formatter,
        formatter.add_label("conflict")?;
        writeln!(formatter, "These remote branches have conflicts:")?;
        formatter.remove_label()?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "{}@{}", branch_name, remote_name)?;
            formatter.remove_label()?;
            writeln!(formatter)?;
            formatter,
    if let Some(wc_commit) = &maybe_checkout {
        let parent_tree = wc_commit.parents()[0].tree();
        let tree = wc_commit.tree();
            formatter.write_str("The working copy is clean\n")?;
            formatter.write_str("Working copy changes:\n")?;
                formatter,
            formatter.add_label("conflict")?;
            writeln!(formatter, "There are unresolved conflicts at these paths:")?;
            formatter.remove_label()?;
                writeln!(formatter, "{}", &workspace_command.format_file_path(&path))?;
    let default_revset = ui.settings().default_revset();
    let revset_expression = revset::parse(args.revisions.as_ref().unwrap_or(&default_revset))?;
    formatter.add_label("log")?;
                let mut formatter = ui.new_formatter(&mut buffer);
                    formatter.add_label("working_copy")?;
                let mut formatter = ui.new_formatter(&mut buffer);
    formatter.add_label("log")?;
                let mut formatter = ui.new_formatter(&mut buffer);
                let mut formatter = ui.new_formatter(&mut buffer);
    write_commit_summary(
        ui.stdout_formatter().as_mut(),
        ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        |formatter: &mut dyn Formatter, target: Option<&RefTarget>| -> Result<(), CommandError> {
                    write!(formatter, ": ")?;
                    write_commit_summary(
                        formatter,
                        repo.as_repo_ref(),
                        &workspace_id,
                        &commit,
                        ui.settings(),
                    )?;
                    writeln!(formatter)?;
                    write!(formatter, " ")?;
                    formatter.add_label("conflict")?;
                    write!(formatter, "(conflicted)")?;
                    formatter.remove_label()?;
                    writeln!(formatter, ":")?;
                        write!(formatter, "  - ")?;
                        write_commit_summary(
                            formatter,
                            repo.as_repo_ref(),
                            &workspace_id,
                            &commit,
                            ui.settings(),
                        )?;
                        writeln!(formatter)?;
                        write!(formatter, "  + ")?;
                        write_commit_summary(
                            formatter,
                            repo.as_repo_ref(),
                            &workspace_id,
                            &commit,
                            ui.settings(),
                        )?;
                        writeln!(formatter)?;
                    writeln!(formatter, " (deleted)")?;
    let mut formatter = ui.stdout_formatter();
    let formatter = formatter.as_mut();
        formatter.add_label("branch")?;
        write!(formatter, "{}", name)?;
        formatter.remove_label()?;
        print_branch_target(formatter, branch_target.local_target.as_ref())?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "@{}", remote)?;
            formatter.remove_label()?;
                    write!(formatter, " (ahead by {} commits)", remote_ahead_count)?;
                    write!(formatter, " (behind by {} commits)", local_ahead_count)?;
                        formatter,
            print_branch_target(formatter, Some(remote_target))?;
            timestamp.timestamp.0.div_euclid(1000),
            (timestamp.timestamp.0.rem_euclid(1000)) as u32 * 1000000,
            formatter.add_label("id")?;
            formatter.add_label("user")?;
            formatter.add_label("time")?;
            formatter.add_label("description")?;
                formatter.add_label("tags")?;
            let mut formatter = ui.new_formatter(&mut buffer);
            formatter.add_label("op-log")?;
                formatter.add_label("head")?;
    let new_wc_commit = if let Some(old_checkout_id) = new_workspace_command
        &new_wc_commit,
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            repo.as_repo_ref(),
            workspace_id,
            &commit,
            ui.settings(),
        )?;
            let ui_path = workspace_command.format_file_path(path);
                new_patterns.extend(locked_wc.sparse_patterns().iter().cloned());
    let mut force_pushed_branches = hashset! {};
            if force {
                force_pushed_branches.insert(branch_name.to_string());
            }
                if force_pushed_branches.contains(branch_name) {
                    writeln!(
                        ui,
                        "  Force branch {branch_name} from {} to {}",
                        short_commit_hash(old_target),
                        short_commit_hash(new_target)
                    )?;
                } else {
                    writeln!(
                        ui,
                        "  Move branch {branch_name} from {} to {}",
                        short_commit_hash(old_target),
                        short_commit_hash(new_target)
                    )?;
                }
pub fn default_app() -> clap::Command {
        default_app().debug_assert();
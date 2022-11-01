use std::time::Instant;
use jujutsu_lib::op_store::{BranchTarget, RefTarget, WorkspaceId};
use jujutsu_lib::repo::{MutableRepo, ReadonlyRepo, RepoRef};
use jujutsu_lib::{conflicts, diff, file_util, files, git, revset, tree};
    print_checkout_stats, resolve_base_revs, short_commit_description, short_commit_hash,
    write_commit_summary, Args, CommandError, CommandHelper, WorkspaceCommandHelper,
use crate::progress::Progress;
    /// or `@ | (remote_branches() | tags()).. | ((remote_branches() |
    /// tags())..)-` if it is not set.
    /// Forget everything about a branch, including its local and remote
    /// targets.
    ///
    /// A forgotten branch will not impact remotes on future pushes. It will be
    /// recreated on future pulls if it still exists in the remote.
        #[arg(long, short = 'B')]
    Rename(GitRemoteRenameArgs),
/// Rename a Git remote
#[derive(clap::Args, Clone, Debug)]
struct GitRemoteRenameArgs {
    /// The name of an existing remote
    old: String,
    /// The desired name for `old`
    new: String,
}

/// By default, pushes any branches pointing to `@`, or `@-` if no branches
/// point to `@`. Use `--branch` to push a specific branch. Use `--all` to push
/// all branches. Use `--change` to generate a branch name based on a specific
/// commit's change ID.
        return Err(UserError(
        workspace_command.snapshot(ui)?;
        if !ui.settings().allow_native_backend() {
            return Err(UserError(
                "The native backend is disallowed by default. Did you mean to pass `--git`?
Set `ui.allow-init-native` to allow initializing a repo with the native backend."
                    .to_string(),
            ));
        }
    let relative_wc_path = file_util::relative_path(&cwd, &wc_path);
    if args.git && wc_path.join(".git").exists() {
        ui.write_warn(format!(
            "Empty repo created. To create repo backed by existing Git repo, run `jj init \
             --git-repo={}` instead.\n",
            relative_wc_path.display()
        ))?;
    }
                // Root is never open
                tx.mut_repo().edit(workspace_id, &target).unwrap();
                tx.mut_repo().edit(workspace_id, &new_commit).unwrap();
        tx.mut_repo().edit(workspace_id, &new_commit).unwrap();
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
            return Err(UserError(message));
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
    let path = workspace_command.parse_file_path(&args.path)?;
            return Err(UserError("No such path".to_string()));
            return Err(UserError("Path exists but is not a file".to_string()));
        formatter.with_label("removed", |formatter| {
            formatter.write_bytes(format!("{:>4}", diff_line.left_line_number).as_bytes())
        })?;
        formatter.with_label("added", |formatter| {
            formatter.write_bytes(format!("{:>4}", diff_line.right_line_number).as_bytes())
        })?;
                    formatter.with_label("removed", |formatter| formatter.write_bytes(before))?;
                    formatter.with_label("added", |formatter| formatter.write_bytes(after))?;
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
                formatter.with_label("header", |formatter| {
                    formatter.write_str(&format!("Added {} {}:\n", description, ui_path))
                })?;
                formatter.with_label("header", |formatter| {
                    formatter.write_str(&format!("{} {}:\n", description, ui_path))
                })?;
                formatter.with_label("header", |formatter| {
                    formatter.write_str(&format!("Removed {} {}:\n", description, ui_path))
                })?;
        formatter.with_label("hunk_header", |formatter| {
            writeln!(
                formatter,
                "@@ -{},{} +{},{} @@",
                hunk.left_line_range.start,
                hunk.left_line_range.len(),
                hunk.right_line_range.start,
                hunk.right_line_range.len()
            )
        })?;
                    formatter.with_label("context", |formatter| {
                        formatter.write_str(" ")?;
                        formatter.write_all(content)
                    })?;
                    formatter.with_label("removed", |formatter| {
                        formatter.write_str("-")?;
                        formatter.write_all(content)
                    })?;
                    formatter.with_label("added", |formatter| {
                        formatter.write_str("+")?;
                        formatter.write_all(content)
                    })?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{} b/{}", path_string, path_string)?;
                    writeln!(formatter, "new file mode {}", &right_part.mode)?;
                    writeln!(formatter, "index 0000000000..{}", &right_part.hash)?;
                    writeln!(formatter, "--- /dev/null")?;
                    writeln!(formatter, "+++ b/{}", path_string)
                })?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{} b/{}", path_string, path_string)?;
                    if left_part.mode != right_part.mode {
                        writeln!(formatter, "old mode {}", &left_part.mode)?;
                        writeln!(formatter, "new mode {}", &right_part.mode)?;
                        if left_part.hash != right_part.hash {
                            writeln!(formatter, "index {}...{}", &left_part.hash, right_part.hash)?;
                        }
                    } else if left_part.hash != right_part.hash {
                        writeln!(
                            formatter,
                            "index {}...{} {}",
                            &left_part.hash, right_part.hash, left_part.mode
                        )?;
                    if left_part.content != right_part.content {
                        writeln!(formatter, "--- a/{}", path_string)?;
                        writeln!(formatter, "+++ b/{}", path_string)?;
                    }
                    Ok(())
                })?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{} b/{}", path_string, path_string)?;
                    writeln!(formatter, "deleted file mode {}", &left_part.mode)?;
                    writeln!(formatter, "index {}..0000000000", &left_part.hash)?;
                    writeln!(formatter, "--- a/{}", path_string)?;
                    writeln!(formatter, "+++ /dev/null")
                })?;
    formatter.with_label("diff", |formatter| {
        for (repo_path, diff) in tree_diff {
            match diff {
                tree::Diff::Modified(_, _) => {
                    formatter.with_label("modified", |formatter| {
                        writeln!(
                            formatter,
                            "M {}",
                            workspace_command.format_file_path(&repo_path)
                        )
                    })?;
                }
                tree::Diff::Added(_) => {
                    formatter.with_label("added", |formatter| {
                        writeln!(
                            formatter,
                            "A {}",
                            workspace_command.format_file_path(&repo_path)
                        )
                    })?;
                }
                tree::Diff::Removed(_) => {
                    formatter.with_label("removed", |formatter| {
                        writeln!(
                            formatter,
                            "R {}",
                            workspace_command.format_file_path(&repo_path)
                        )
                    })?;
                }
        Ok(())
    })
        formatter.with_label("conflict", |formatter| {
            writeln!(formatter, "These branches have conflicts:")
        })?;
            formatter.with_label("branch", |formatter| write!(formatter, "{}", branch_name))?;
        formatter.with_label("conflict", |formatter| {
            writeln!(formatter, "These remote branches have conflicts:")
        })?;
            formatter.with_label("branch", |formatter| {
                write!(formatter, "{}@{}", branch_name, remote_name)
            })?;
            formatter.with_label("conflict", |formatter| {
                writeln!(formatter, "There are unresolved conflicts at these paths:")
            })?;
    let revset_expression =
        workspace_command.parse_revset(args.revisions.as_ref().unwrap_or(&default_revset))?;
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
    let mut revset = workspace_command.evaluate_revset(&revset_expression)?;
                    formatter.with_label("working_copy", |formatter| {
                        template.format(&commit, formatter)
                    })?;
                } else {
                    template.format(&commit, formatter.as_mut())?;
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
        .map_err(|_| UserError(format!("Failed to run editor '{editor}'")))?;
        return Err(UserError(format!("Editor '{editor}' exited with an error")));
            tx.mut_repo().edit(workspace_id, &new_checkout).unwrap();
        tx.mut_repo().edit(workspace_id, &new_commit)?;
    tx.mut_repo().edit(workspace_id, &new_commit).unwrap();
        return Err(UserError(String::from(
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
        return Err(UserError(String::from("No changes to move")));
        return Err(UserError(String::from("Cannot squash merge commits")));
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
        return Err(UserError(String::from("No changes selected")));
        return Err(UserError(String::from("Cannot unsquash merge commits")));
            return Err(UserError(String::from("No changes selected")));
        let matcher = workspace_command.matcher_from_values(&args.paths)?;
(parent) commit. The remainder will be in the second commit. If you
don't make any changes, then the operation will be aborted.
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
    for root_result in workspace_command
        .evaluate_revset(&roots_expression)
    for child_commit in workspace_command
        .evaluate_revset(&children_expression)
        let new_child_parents_expression = RevsetExpression::Difference(
        );
        let new_child_parents: Result<Vec<Commit>, BackendError> = workspace_command
            .evaluate_revset(&new_child_parents_expression)
            .unwrap()
            .iter()
            .commits(store)
            .collect();
            return Err(UserError(format!(
                return Err(UserError(format!("No such branch: {}", branch_name)));
                    Some(_) => Err(UserError(format!(
                return Err(UserError(
    let print_branch_target = |formatter: &mut dyn Formatter,
                               target: Option<&RefTarget>|
     -> Result<(), CommandError> {
        match target {
            Some(RefTarget::Normal(id)) => {
                write!(formatter, ": ")?;
                let commit = repo.store().get_commit(id)?;
                write_commit_summary(
                    formatter,
                    repo.as_repo_ref(),
                    &workspace_id,
                    &commit,
                    ui.settings(),
                )?;
                writeln!(formatter)?;
            }
            Some(RefTarget::Conflict { adds, removes }) => {
                write!(formatter, " ")?;
                formatter.with_label("conflict", |formatter| write!(formatter, "(conflicted)"))?;
                writeln!(formatter, ":")?;
                for id in removes {
                    write!(formatter, "  - ")?;
                for id in adds {
                    let commit = repo.store().get_commit(id)?;
                    write!(formatter, "  + ")?;
                    write_commit_summary(
                        formatter,
                        repo.as_repo_ref(),
                        &workspace_id,
                        &commit,
                        ui.settings(),
                    )?;
                    writeln!(formatter)?;
            None => {
                writeln!(formatter, " (deleted)")?;
            }
        }
        Ok(())
    };
        formatter.with_label("branch", |formatter| write!(formatter, "{}", name))?;
            formatter.with_label("branch", |formatter| write!(formatter, "@{}", remote))?;
            formatter.with_label("id", |formatter| formatter.write_str(&op.id().hex()[0..12]))?;
            formatter.with_label("user", |formatter| {
                formatter.write_str(&format!("{}@{}", metadata.username, metadata.hostname))
            })?;
            formatter.with_label("time", |formatter| {
                formatter.write_str(&format!(
                    "{} - {}",
                    format_timestamp(&metadata.start_time),
                    format_timestamp(&metadata.end_time)
                ))
            })?;
            formatter.with_label("description", |formatter| {
                formatter.write_str(&metadata.description)
            })?;
                formatter.with_label("tags", |formatter| {
                    formatter.write_str(&format!("\n{}: {}", key, value))
                })?;
            formatter.with_label("op-log", |formatter| {
                if is_head_op {
                    formatter.with_label("head", |formatter| template.format(&op, formatter))
                } else {
                    template.format(&op, formatter)
                }
            })?;
        return Err(UserError("Cannot undo a merge operation".to_string()));
        return Err(UserError("Cannot undo repo initialization".to_string()));
        return Err(UserError("Workspace already exists".to_string()));
        file_util::relative_path(old_workspace_command.workspace_root(), &destination_path)
            .display()
    let mut new_workspace_command = WorkspaceCommandHelper::new(
        let paths_to_add = args
            .add
            .iter()
            .map(|v| workspace_command.parse_file_path(v))
            .collect::<Result<Vec<_>, _>>()?;
        let paths_to_remove = args
            .remove
            .iter()
            .map(|v| workspace_command.parse_file_path(v))
            .collect::<Result<Vec<_>, _>>()?;
        None => Err(UserError(
        return Err(UserError("Remote already exists".to_string()));
        .map_err(|err| UserError(err.to_string()))?;
        return Err(UserError("Remote doesn't exist".to_string()));
        .map_err(|err| UserError(err.to_string()))?;
fn cmd_git_remote_rename(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &GitRemoteRenameArgs,
) -> Result<(), CommandError> {
    let mut workspace_command = command.workspace_helper(ui)?;
    let repo = workspace_command.repo();
    let git_repo = get_git_repo(repo.store())?;
    if git_repo.find_remote(&args.old).is_err() {
        return Err(UserError("Remote doesn't exist".to_string()));
    }
    git_repo
        .remote_rename(&args.old, &args.new)
        .map_err(|err| UserError(err.to_string()))?;
    let mut tx = workspace_command
        .start_transaction(&format!("rename git remote {} to {}", &args.old, &args.new));
    tx.mut_repo().rename_remote(&args.old, &args.new);
    if tx.mut_repo().has_changes() {
        workspace_command.finish_transaction(ui, tx)?;
    }
    Ok(())
}

    git_fetch(ui, tx.mut_repo(), &git_repo, &args.remote)
        .map_err(|err| UserError(err.to_string()))?;
        return Err(UserError(
            UserError("No destination specified and wasn't able to guess it".to_string())
            return Err(UserError(
        git_fetch(ui, fetch_tx.mut_repo(), &git_repo, remote_name).map_err(|err| match err {
            GitFetchError::InternalGitError(err) => UserError(format!("Fetch failed: {err}")),
// Wrapper around git::fetch that adds progress feedback on TTYs
fn git_fetch(
    ui: &mut Ui,
    mut_repo: &mut MutableRepo,
    git_repo: &git2::Repository,
    remote_name: &str,
) -> Result<Option<String>, GitFetchError> {
    let mut callback = None;
    if ui.use_progress_indicator() {
        let mut progress = Progress::new(Instant::now(), ui);
        callback = Some(move |x: &git::Progress| {
            progress.update(Instant::now(), x);
        });
    }
    let result = git::fetch(
        mut_repo,
        git_repo,
        remote_name,
        callback
            .as_mut()
            .map(|x| x as &mut dyn FnMut(&git::Progress)),
    );
    result
}

                fn find_branches_targeting<'a>(
                    view: &'a View,
                    target: &RefTarget,
                ) -> Vec<(&'a String, &'a BranchTarget)> {
                    view.branches()
                        .iter()
                        .filter(|(_, branch_target)| {
                            branch_target.local_target.as_ref() == Some(target)
                        })
                        .collect()
                }

                // Search for branches targeting @
                let mut branches = find_branches_targeting(
                    workspace_command.repo().view(),
                    &RefTarget::Normal(checkout.clone()),
                );
                if branches.is_empty() {
                    // Try @- instead if it has exactly one parent, such as after `jj squash`
                    let commit = workspace_command.repo().store().get_commit(checkout)?;
                    if let [parent] = commit.parent_ids() {
                        branches = find_branches_targeting(
                            workspace_command.repo().view(),
                            &RefTarget::Normal(parent.clone()),
                        );
                    }
                }
                for (branch_name, branch_target) in branches {
                    let push_action = classify_branch_push_action(branch_target, &args.remote);
                    match push_action {
                        BranchPushAction::AlreadyMatches => {}
                        BranchPushAction::LocalConflicted => {}
                        BranchPushAction::RemoteConflicted => {}
                        BranchPushAction::Update(update) => {
                            branch_updates.push((branch_name.clone(), update));
        .map_err(|err| UserError(err.to_string()))?;
        .ok_or_else(|| UserError(format!("Branch {} doesn't exist", branch_name)))?;
        BranchPushAction::LocalConflicted => {
            Err(UserError(format!("Branch {} is conflicted", branch_name)))
        }
        BranchPushAction::RemoteConflicted => Err(UserError(format!(
        GitCommands::Remote(GitRemoteCommands::Rename(command_matches)) => {
            cmd_git_remote_rename(ui, command, command_matches)
        }
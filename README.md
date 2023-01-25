# Sysnap WS22

## Authors

- Michael Engel (michael.engel@uni-bamberg.de)
- Timo Renk (timo.renk@stud.uni-bamberg.de)
- Fabian Adam (fabian-david.adam@stud.uni-bamberg.de)
- Leonhard Kohn (leonhard.kohn@stud.uni-bamberg.de)
- Tobias Treuheit (tobias-niklas.treuheit@stud.uni-bamberg.de))
- Max Meidinger (max_meidinger@stud.uni-bamberg.de)

## Description

In this project, we attempt to build a mikrokernel in RUST on a RISCV processor. For this we use the code of Timo Renk as a basis.

To run the code, the following is needed:

- gdb-multiarch or riscv-elf-gdb
- rust with rustup and cargo
- qemu for riscv

## Running the project
Once all the required tools are installed, the project can be compiled and run using the provided tasks (Shortcut `Strg+Alt+R`).

- Use `Build riscv_rust_os` to build the binaries for the kernel.
- Use `Build user binaries` to build the binaries for the user processes to be run
- Finally, use `Debug riscv_rust_os` to fire up qemu with the compiled kernel

Now qemu is running with the given binary! But in orderr to get anything from the emulator, the visual studio debugger needs to connect to the debug server.
How to connect to the server is already set up in launch.json, so simply pressing `F5` should suffice to connect to the debug server.

To see what instruction is executed at the moment, you can open the `Disassembly View` using the Command Palette (Keyboard Shortcut: `F1`). 

#


## Proposed Git Workflow for this project
The following workflow might be a bit excessive for the scale of our project, but adhering to it will help to understand git better and to be more comfortable in its usage.

For a more detailed explanation of mentioned commands address the short explanation of them down below.

#

### Implementing new feature
You would typically start by creating a new feature branch from the current state of the master branch. 
The next step would be to iteratively implement the feature. This might require more or less changes and should depending on the number of changes be split into multiple commits.
For bigger features this might create a lot of smaller commits. 

Each commit should, if possible represent some sort of logical unit, meaning that the changes should serve a common purpose as part of the implementation of the feature.

After implementing the feature, all the changes need to be brought into the master branch.

### Include changes into the master branch
One way to do this might be to change to the master branch and merge the feature branch into the master branch.
This will create a merge commit every time, polluting the history of the master branch with the superfluous merge commits. 

This might be avoided by rebasing.

### Rebasing

Once you are finished implementing your feature, there might already be some new changes to the master branch. If thats the case, your local branch and the remote master are diverging by some amount of commits.  
This might look like this:
```
          A---B---C feature
         /
    D---E---F---G master
```
Rebasing the feature branch onto the curret state of the master-branch will re-apply the commits of the feature-branch on top of the newest commits of the master-branch.  
If done successfully, the local state of the feature branch will look like this:
```
                  A'---B'---C' feature
                 /
    D---E---F---G master
```
But of course some changes introduced in the commits of the feature-branch might conflict with the diverging conflicts of the master-branch. If that is the case, you will have to resolve the conflicts and recommit the changes for that specific commit.  
The interactive rebasing mode is pretty useful for that.

### Pushing the rebased branch to the remote repository

Now if you try to push your rebased branch that is ready if integration into the master to the remote feature-branch, you will notice that git complains about your local branch and the remote branch having diverged.  
This is due to the reason, that we created a whole new set of commits when we rebased the feature-branch on top of the most recent state of the master branch.  
So from the perspective of the VCS, this new state of the local branch is completeley different from the remote state, because the history of the local branch noew includes the newer commits of the master-branch, which are not part of the remote branches history.

But, working with GitLab, we need to get our local state to the remote repository in order the merge the changes into the master branch using a *merge request*.  
So how do we deal with that?

One way would be to force the current, rebased state of the feature branch onto the remote feature-branch. But this is quite destructive and might overwrite recent changes from other programmers.

The "cleaner" way would be to create an additional branch before rebasing and then rebase using this new "integration"-branch (naming convention: "integration/"\<original branch name\>).

Once rebase, this new branch can then be pushed to the remote repository without any conflicts, since the branch did not exist there yet.

We can then continue with the *merge request* to finally introduce the changes into the master branch.

### Merge Request
The final step is to create a *merge request*. This should only happen if the feature is well-tested, which in our case probably means testing the feature by hand.

First, you need to push the branch to the remote repository. From there you can create a merge request into the master branch.

Make sure to reference the resolved issue in the title of the Merge Request and assign collaborators on that issue as reviewers, so that they can take a finally look at the proposed changes.

The given merge options can in most cases stay ticked.

Especially the squashing of the commits is quite useful:
It squashes all commits of the feature branch together into one with the name of the merge request. This results in a quite nice commit history, consisting of one commit for each issue/feature.

#

## Short overview of useful Git Commands
Here is a list of useful git commands in the order in which they might be used in a typical workflow.

### Cloning the repository
`git clone <upstream-url>`
### Creating a new branch
`git checkout -b <branchname>`
This will create a new local branch with the name `<branchname>`. This new branch will initially have the same state as the branch that it was created from. 
### Checking files with changes
`git status`
This command will show a number of things:

- Which branch you are currently on
- whether and by how many commits the remote or the local branches are ahead of each other and whether they diverge
- Which files are staged for commit (in green)
- Which files are not staged for commit (in red)

### Staging files for a commit
`git add <filepath>`

This command will stage all changed files that are included in the given path (recursively!) to be added to the next commit

### Unstaging files
`git reset -- <filepath>`

This command will remove all files in the staging area included in the file path.

### Creating a new commit with staged files
`git commit -m <commitmessage>` 

This will create a new commit containing all the previously staged changes.

### Adding changes to a previous commit
`git commit --amend -m <commitmsg>`

This will add all currently staged changes to the previous commit. Without the `-m` flag followed by a commit message a editor will be opened, in which you can edit the previous commit message. 

### Pushing changes to a remote repository
`git push`

This will push the locally committed changes to the remote branch. This might not work when there is no branch set up as a remote branch for the local branch, for example when the current working branch is created locally an there is no remote branch tracking the local branch yet.

Using `git push -u <remote> <branchname>` will set the upstream branch that should correspond to the local branch. If that branch does not exist yet, it will be created.

`<remote>` corresponds to the name of the upstream repository. In our case there will probably be only one defaulting to the name `origin`.

`<branchname>` is the name of the remote branch that is supposed to be tracking the local branch. Most of the time it makes sense to give it the same name as the local branch.

### Get changes from the remote repository
`git fetch`

This command fetches the current changes from the remote branch, but does not integrate them into the local state.

### Merge remote changes and local changes
`git merge`

This will try to automatically combine all remote changes that have previously been fetched with the local committed changes. There might, however, be some conflicts when the same file has been edited in different commits. Those will need to be resolved and committed in order to finalize the merge.

`git merge <branchname>`

If you want to merge another local branch into the working branch you can use this command to merge be branch `<branchname>` into the current working branch.

### Fetch + Merge
`get pull`

Combines `git fetch` and `git merge` into one.

### Rebase
`git rebase <branchname>`

This will apply and recalculate all changes that have happened since the current working branch diverged from the path given by `<branchname>` on top of the current state of `<branchname>`.
This might also create conflicts that will need to be resolved.


## License

This is a University Project.

#

#



# Risc-V Rust OS

Os written in Rust

## Install

- rustup target install riscv64gc-unknown-none-elf

### GDB

gdb-multiarch/ riscv-elf-gdb

#### Windows

msys2: -> pacman -S mingw-w64-x86_64-toolchain

### objcopy

cargo install cargo-binutils
rustup component add llvm-tools-preview

## RISC-V

[Register]<https://en.wikichip.org/wiki/risc-v/registers>
<https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#system-reset-extension-eid-0x53525354-srst>
<https://github.com/riscv/riscv-isa-manual/#readme>
<https://github.com/rust-embedded/riscv>
[Register]

## UART

<https://osblog.stephenmarz.com/ch0.html>
<https://os.phil-opp.com/>
<https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds>
<https://github.com/skyzh/core-os-riscv/blob/master/kernel/src/uart.rs>
<https://docs.rust-embedded.org/book/start/qemu.html>

UART
<https://www.lammertbies.nl/comm/info/serial-uart>

Check riscv reader for paper info for register infos in first two lectures

## Plic

<https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

## Questions

- How to avoid race-conditions in UART/ Kernel?
- What is mtval?

### Answered

- Why align to 16?
  - `ALIGN(4096) tells the linker to align the current memory location (which is
       0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
       system's resolution is 4,096 bytes or 4 KiB.`
- >ram AT>ram?
- sdata .sbss
- use wfi?
  - Wait for interrupts

## GDB

- info registers

<https://stackoverflow.com/questions/2420813/using-gdb-to-single-step-assembly-code-outside-specified-executable-causes-error>

- gdbtui. Or run gdb with the -tui switch. Or press C-x C-a after entering gdb.
- layout asm
- Press C-x s
- use si ni
- use gdb-multiarch!
- x/100x $sp
- -exec p/x $mepc

readelf -a user_1 | less

## LLDB

Don't use it!
<https://lldb.llvm.org/use/map.html>

## Tools

### NM



Check memory layout
```x86_64-w64-mingw32-gcc-nm riscv_rust_os | sort```

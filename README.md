Diablo II: Resurrected: Awesome Launcher

Usage: d2ral.exe [OPTIONS] <COMMAND>

Commands:
    volley        D2RAL.exe volley => START ALL THE PROFILES!!!
    start         D2RAL.exe -n {profile_name} start =>  Start a profile
    list          D2RAL.exe list => List Stored Profiles
    display       D2RAL.exe Display {profile_name} => Display Stored Profile Details
    add           D2RAL.exe -n {profile_name} -u {profile_username} -p {profile_password} -r {region} add => Add a profile
    delete        D2RAL.exe -n {profile_name} delete => Delete a profile
    edit          D2RAL.exe -n {profile_name} update => update a profile with new options
    copy          D2RAL.exe -n {profile_name} copy {new profile name}=> copy a profile with new options
    close-handle  D2RAL.exe handle => Kill Mulisession mutex handle
    set-title     D2RAL.exe set-title {new title} => Set a default title window to a new title
    custom-title  D2RAL.exe custom-title {old} {new}
    example       colored examples
    interactive   "shell" mode
    help          Print this message or the help of the given subcommand(s)

Options:
    -n, --name-profile <NAME>          [required for D2RAL Add,Delete,Set-Title] [default: ]
    -u, --username-profile <USERNAME>  [required for command: Add](Optional for D2RAL: Edit, Copy) [default: ]
    -p, --password-profile <PASSWORD>  [required for command Add](Optional for D2RAL: Edit, Copy) [default: ]
    -r, --region-profile <REGION>      [default: ]
    -m, --mode-launch <MODE>           [ "none"=Profile , "normal", "direct", "txtdirect" , "{mode}"=-mod {mode} -txt ] (Add, Edit, Copy, Start, Volley) [default: none]
    -s, --sound-launch <SOUND>         ["0"=Profile "1"=Sound, "2"=No Sound] (Add, Edit, Copy, Start, Volley) [default: 0]
    -w, --window-launch <WINDOW>       (Optional for D2RAL: Add, Edit, Copy, Start, Volley) [fullscreen:1 , windowed:2] [default: 0]
    -i, --inject <DLL>                 inject (Optional for D2RAL: Start, Volley) [path to dll] [default: ]
    -h, --help                         Print help information
    -V, --version                      Print version information
    
Use the flag options to set the parameters of each command
    Examples>>>
    Add a profile with region set to 'us', sound off, window mode, and mod 'blockhd'
    >D2RAL.exe -n profile1 -u user@gmail.com -p 12345 -r us -s 2 -w 1 -m blockhd add
    Start it
    >D2RAL.exe -n profile1 start
    Start it with injection
    >D2RAL.exe -n profile1 -i path\to\dll start
    Edit the profile with region to eu, sound on, window fullscreen, and '-direct -txt'
    >D2RAL.exe -n profile1 -u user@gmail.com -p 12345 -r eu -s 1 -w 1 -m txtdirect edit
    Copy and edit it changine launch mode and setting to windowed
    >D2RAL.exe -n profile1 -w 1 copy profile2
    Copy and edit it more
    >D2RAL.exe -n profile1 -u user2@gmail.com -p 54321 -m normal copy profile3
    Start all profiles with injection
    >D2RAL.exe -i path\to\dll volley
    Delete them
    >D2RAL.exe -n profile1 delete
    >D2RAL.exe -n profile2 delete
    >D2RAL.exe -n profile3 delete

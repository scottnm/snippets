Param(
    [Parameter(Mandatory=$true)]
    [string]$ProjectFolder,

    [Parameter(Mandatory=$true)]
    [string]$Name
    )

cp -rec $PSScriptRoot\template_project $ProjectFolder\$Name

pushd $ProjectFolder\$Name

dir vs_quick.* | Rename-Item -NewName { $_.name -replace "vs_quick",$Name }
$files = dir "$Name.*","build.ps1"

function ReplaceInFiles {
    Param(
        [string[]]$FileSet,
        [string]$Old,
        [string]$New
        )

    $FileSet | %{ (Get-Content $_).replace($Old, $New) | Set-Content $_ }
}

# REPLACE REFERENCES TO vs_quick with $Name
ReplaceInFiles -FileSet $files -Old "vs_quick" -New $Name
ReplaceInFiles -FileSet $files -Old "vsquick" -New $Name

# REPLACE GUIDS
$files |
    sls "\w{8}-\w{4}-\w{4}-\w{4}-\w{12}" |
    %{ $_.Matches.Groups[0].Value } |
    sort |
    Get-Unique -AsString |
    %{
        ReplaceInFiles -FileSet $files -Old $_ -New (new-guid)
    }

popd


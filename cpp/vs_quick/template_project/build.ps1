Param(
    [ValidateSet("Debug", "Release")]
    [string]$Configuration="Debug"
    )

MSBuild vs_quick.sln -verbosity:m -p:Configuration=$Configuration

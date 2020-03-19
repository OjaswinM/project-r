; a small assembly file that is linked at the beginning of the kernel. Its
; job is to simply call the main() routine of C which is the entry point of
; the kernel
[bits 32]		; since we are in protected mode
[extern mail]		; define main() as external 

call mail		; call main() which is linked here from C code
jmp $

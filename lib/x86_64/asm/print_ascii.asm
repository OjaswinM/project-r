print_ascii:
; Print the null terminated string whose start is pointed by the bx register. The bois 
; interrupt for printing is called with ah=0x0e and al=*ascii of char to print*
	pusha
	mov ah, 0x0e

print_char:
	cmp byte [bx], 0x00
	je exit
	mov al, [bx]
	int 0x10
	add bx, 0x01
	jmp print_char

exit:
	popa
	ret


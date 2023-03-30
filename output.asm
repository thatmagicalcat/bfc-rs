section .data
    array times 2000 db 0
    newline db 0xA
    buffer times 2 db 0
    err_msg_zero db "Error '-': Cannot decrement zero", 10
    err_msg_zero_len equ $-err_msg_zero
    err_msg_max db "Error '+': Cannot increment more than 255", 10
    err_msg_max_len equ $-err_msg_max
    err_msg_last db "Error '>': already at the last cell", 10
    err_msg_last_len equ $-err_msg_last
    err_msg_first db "Error '<': already at the first cell", 10
    err_msg_first_len equ $-err_msg_first

section .text
    global _start

putChar:
    mov eax, 1
    mov edi, 1
    mov esi, buffer
    mov edx, 1
    syscall
    ret

getChar:
    mov eax, 0
    mov edi, 0
    mov esi, buffer
    mov edx, 2
    syscall
    ret

_start:
    mov ebx, array
    mov edx, 0
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 7
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_1:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_1
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 2
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_2:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_2
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 1
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_3:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_3
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 8
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_4:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_4
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 8
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 11
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_5:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_5
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 1
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 3
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_6:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_6
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 2
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 8
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_7:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_7
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 7
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 11
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_8:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_8
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 1
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 11
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_9:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_9
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 4
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_10:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_10
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 8
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_11:
    push rcx
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 1
    add edx, 1
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, 1
    mov byte [ebx], al
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_11
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
    ;; ---- Shift Right ----
    cmp edx, 2000
    jz label_err_msg_last
    add ebx, 2
    add edx, 2
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, 10
    mov byte [ebx], al
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar
exit_program:
    mov rax, 60
    mov rdi, 0
    syscall
label_err_msg_zero:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_zero
    mov edx, err_msg_zero_len
    syscall
    jmp exit_program
label_err_msg_max:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_max
    mov edx, err_msg_max_len
    syscall
    jmp exit_program
label_err_msg_last:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_last
    mov edx, err_msg_last_len
    syscall
    jmp exit_program
label_err_msg_first:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_first
    mov edx, err_msg_first_len
    syscall
    jmp exit_program
#include "cpcemu_public.h"
#include "cpcemu_parser_bindings.h"
#include <stdio.h>
#include <memory.h>

int cpcemu_mem_read_at(
	const unsigned short *address_value, const char *address_symbol)
{
	printf("emu read args there %p %p \n", address_value, address_symbol);
}
int cpcemu_mem_write_at(
	const unsigned short *address_value, const char *address_symbol,
	const unsigned char *value_value, const char *value_symbol)
{
	printf("emu read args there %p %p %p %p\n", address_value, address_symbol, value_value, value_symbol);
}

int cpcemu_brk_add_z80_brk_at(
	const unsigned short *address_value, const char *address_symbol,
	const struct breakpoint_condition *cond)
{
	printf("emu read args there %p %p %p\n", address_value, address_symbol, cond);
}

int main(int argc, char **argv)
{
	if (argc == 1)
	{
		launch_tests();
		return 0;
	}
	if (argc != 2)
	{
		printf("[ERROR] Usage: %s <[command in one argument>]\n", argv[0]);
		return -1;
	}

	char *line = argv[1];
	printf(">> Try to execute: %s\n", line);
	const unsigned char *res = cpcemu_execute_line(line);
	if (NULL == res)
	{
		printf("No error\n");
	}
	else
	{
		printf("[ERROR] %s\n", res);
		free((void *)res);
	}
}
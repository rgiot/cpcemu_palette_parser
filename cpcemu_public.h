#ifndef CPCEMU_PUBLIC
#define CPCEMU_PUBLIC

#define CPCEMU_PUBLIC_VERSION "0.0.1"
#define CPCEMU_SUCCESS 0
#define CPCEMU_FAILURE 1

struct breakpoint_condition
{
	// ??
};

// When arguments can be of different types, they have been duplicated. only one address is not null
// TODO use a better way to handle polymorphism

/**
 * @brief Ask the emulator to read a byte and display it
 * 
 * @param address_value  the address provided as a value
 * @param address_symbol the address provided as label
 * @return int error code
 */
int cpcemu_mem_read_at(
	const unsigned short *address_value, const char *address_symbol);

/**
 * @brief Ask the emulator to write a byte in memory
 * 
 * @param address_value the address provided as a value
 * @param address_symbol the address provided as label
 * @param value_value  the value  provided as a value
 * @param value_symbol the value provided as a label
 * @return int 
 */
int cpcemu_mem_write_at(
	const unsigned short *address_value, const char *address_symbol,
	const unsigned char *value_value, const char *value_symbol);

/**
 * @brief Add a breakpoint
 * 
 * @param address_value the address provided as a value
 * @param address_symbol the address provided as label
 * @param cond the condition of execution of the breakpoint
 * @return int 
 */
int cpcemu_brk_add_z80_brk_at(
	const unsigned short *address_value, const char *address_symbol,
	const struct breakpoint_condition *cond);

// TODO add other methods of interest
#endif
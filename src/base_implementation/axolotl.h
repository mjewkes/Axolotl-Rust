typedef int (*fn_genkeypair)(uint8_t* out_private, uint8_t* out_public, int);
typedef int (*fn_genpubkey)(uint8_t* private,uint8_t* out,int);
typedef int (*fn_dhke)(uint8_t*,uint8_t*,uint8_t*,int);

typedef int (*fn_enc)(uint8_t*, int, uint8_t*, uint8_t*,int, uint8_t*, int*);
typedef int (*fn_dec)(uint8_t*, int, uint8_t*, uint8_t*,int, uint8_t*, int*);

void* create_axolotl_context(fn_genkeypair
									   ,fn_genpubkey
	                                   ,fn_dhke
	                                   ,fn_enc
	                                   ,fn_dec
	                                   ,char* init_info
	                                   ,uint32_t init_info_len
	                                   ,char*  ratchet_info
	                                   ,uint32_t ratchet_info_len
	                                   ,char* msg_info
	                                   ,uint32_t msg_info_len
	                                   );

void destroy_axolotl_context(void* axo);



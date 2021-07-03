#define HEX_NUM 0x10
#define DEC_NUM 10
#define DEC_NUM_COLLIDE 16

int
main(int argc, char**)
{
    switch (argc)
    {
        case 0: return 0;
        case HEX_NUM: return 1;
        case DEC_NUM: return 1;
        case DEC_NUM_COLLIDE: return 0;
        default: return 0;
    }
}

#include <check.h>
#include "Pieces/Piece.h"
#include "Board/Board.h"

START_TEST (test_check_square)
{
    board b;
    board_init(&b);

    fail_unless(check_square(&b.sqrs[0], 0, 0) == false);
}
END_TEST

int main(void)
{
    Suite *s1 = suite_create("Core");
    TCase *tc1_1 = tcase_create("Core");
    SRunner *sr = srunner_create(s1);
    int nf;

    suite_add_tcase(s1, tc1_1);
    tcase_add_test(tc1_1, test_check_square);

    srunner_run_all(sr, CK_ENV);
    nf = srunner_ntests_failed(sr);
    srunner_free(sr);

    return nf == 0 ? 0 : 1;
}
-- Zone 21 Moradon: Apply session 45 (clean original capture) NPC data
-- server_tag=original, server=185.81.239.132
-- Missing templates: 4
-- Missing spawns: 46 protos, 180 spawn points
-- Template updates: 165 protos

-- Step 1: Add missing templates
INSERT INTO npc_template (s_sid, is_monster, str_name, s_pid, s_size, by_type, i_selling_group, s_level)
VALUES
  (31871, false, 'Proto 31871', 26005, 100, 83, 0, 50),
  (31872, false, 'Proto 31872', 26005, 100, 84, 0, 50),
  (31874, false, 'Proto 31874', 26002, 100, 86, 0, 50),
  (31875, false, 'Proto 31875', 26002, 100, 87, 0, 50)
ON CONFLICT (s_sid, is_monster) DO NOTHING;

-- Step 2: Add missing spawns
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES
  (21, 450, true, 1, 571, 555, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 575, 558, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 580, 566, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 587, 570, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 587, 571, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 590, 582, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 595, 559, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 450, true, 1, 597, 562, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 452, true, 1, 208, 738, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 452, true, 1, 210, 718, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 452, true, 1, 210, 734, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 543, 832, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 547, 826, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 558, 843, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 565, 842, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 570, 847, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 572, 855, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 576, 848, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 577, 841, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 582, 844, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 583, 853, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 583, 862, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 585, 853, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 585, 856, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 585, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 553, true, 1, 597, 855, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 603, false, 1, 851, 782, 0, 0, 0, 0, 0, 0, 0, 110, 0, '', 0),
  (21, 652, true, 1, 452, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 454, 855, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 456, 862, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 458, 848, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 460, 870, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 461, 860, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 463, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 463, 878, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 464, 864, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 464, 874, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 465, 856, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 465, 870, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 466, 849, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 466, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 475, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 476, 852, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 480, 850, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 481, 868, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 481, 871, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 496, 874, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 652, true, 1, 496, 882, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 249, 473, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 260, 479, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 271, 470, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 275, 479, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 285, 478, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 313, 170, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 315, 157, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 317, 160, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 318, 158, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 318, 162, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 322, 153, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 322, 161, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 323, 166, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 325, 158, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 325, 161, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 328, 167, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 336, 167, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 344, 162, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 346, 165, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 348, 154, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 653, true, 1, 354, 159, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 285, 926, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 285, 927, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 286, 932, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 289, 928, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 290, 926, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 298, 934, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 300, 940, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 309, 961, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 313, 955, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 318, 944, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 318, 951, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 1, 319, 944, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 353, 943, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 354, 933, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 354, 979, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 359, 972, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 361, 945, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 362, 942, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 364, 965, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 366, 983, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1057, true, 1, 371, 952, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 284, 882, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 288, 885, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 292, 882, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 300, 885, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 303, 876, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 305, 872, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 313, 899, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 324, 885, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 324, 887, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1058, true, 1, 330, 903, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 158, 858, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 158, 933, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 161, 857, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 161, 862, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 164, 814, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 164, 939, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 165, 806, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 167, 855, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 170, 929, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 193, 908, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 194, 903, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1154, true, 1, 202, 913, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 356, 869, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 358, 846, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 360, 853, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 361, 855, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 362, 845, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 383, 874, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 384, 869, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 385, 880, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 386, 880, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 391, 875, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 393, 873, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 397, 862, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1180, true, 1, 398, 888, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1671, true, 1, 340, 712, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8650, true, 1, 312, 909, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8650, true, 1, 365, 973, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8651, true, 1, 375, 159, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8651, true, 1, 379, 160, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8651, true, 1, 382, 157, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8651, true, 1, 384, 166, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8652, true, 1, 63, 437, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8652, true, 1, 103, 681, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8652, true, 1, 107, 689, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 8652, true, 1, 306, 76, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9021, true, 1, 144, 435, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9271, true, 1, 396, 201, 1, 0, 0, 0, 0, 7, 30, 124, 0, '', 0),
  (21, 9272, true, 1, 257, 249, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9272, true, 1, 259, 253, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9272, true, 1, 272, 266, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9272, true, 1, 282, 247, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 328, 239, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 328, 244, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 334, 254, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 337, 256, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 345, 247, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 346, 241, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 9273, true, 1, 347, 247, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 13001, false, 1, 919, 659, 0, 0, 0, 0, 0, 0, 0, 136, 0, '', 0),
  (21, 13015, false, 1, 815, 921, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0),
  (21, 16047, false, 1, 755, 772, 0, 0, 0, 0, 0, 0, 0, 76, 0, '', 0),
  (21, 19017, false, 1, 716, 680, 0, 0, 0, 0, 0, 0, 0, 45, 0, '', 0),
  (21, 19020, false, 1, 241, 325, 0, 0, 0, 0, 0, 0, 0, 180, 0, '', 0),
  (21, 19023, false, 1, 48, 897, 0, 0, 0, 0, 0, 0, 0, 46, 0, '', 0),
  (21, 19052, false, 1, 705, 628, 0, 0, 0, 0, 0, 0, 0, 91, 0, '', 0),
  (21, 19053, false, 1, 709, 646, 0, 0, 0, 0, 0, 0, 0, 44, 0, '', 0),
  (21, 19054, false, 1, 709, 693, 0, 0, 0, 0, 0, 0, 0, 62, 0, '', 0),
  (21, 19055, false, 1, 710, 695, 0, 0, 0, 0, 0, 0, 0, 61, 0, '', 0),
  (21, 19056, false, 1, 818, 768, 0, 0, 0, 0, 0, 0, 0, 135, 0, '', 0),
  (21, 19056, false, 1, 819, 767, 0, 0, 0, 0, 0, 0, 0, 135, 0, '', 0),
  (21, 19056, false, 1, 822, 767, 0, 0, 0, 0, 0, 0, 0, 135, 0, '', 0),
  (21, 19056, false, 1, 825, 766, 0, 0, 0, 0, 0, 0, 0, 135, 0, '', 0),
  (21, 19056, false, 1, 826, 769, 0, 0, 0, 0, 0, 0, 0, 135, 0, '', 0),
  (21, 19063, false, 1, 823, 811, 0, 0, 0, 0, 0, 0, 0, 180, 0, '', 0),
  (21, 24414, false, 1, 343, 880, 0, 0, 0, 0, 0, 0, 0, 49, 0, '', 0),
  (21, 25002, false, 1, 60, 887, 0, 0, 0, 0, 0, 0, 0, 51, 0, '', 0),
  (21, 25158, false, 1, 602, 659, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  (21, 25181, false, 1, 297, 777, 0, 0, 0, 0, 0, 0, 0, 25, 0, '', 0),
  (21, 25182, false, 1, 255, 388, 0, 0, 0, 0, 0, 0, 0, 25, 0, '', 0),
  (21, 25183, false, 1, 350, 902, 0, 0, 0, 0, 0, 0, 0, 25, 0, '', 0),
  (21, 25184, false, 1, 493, 563, 0, 0, 0, 0, 0, 0, 0, 25, 0, '', 0),
  (21, 29028, false, 1, 332, 64, 0, 0, 0, 0, 0, 0, 0, 31, 0, '', 0),
  (21, 31402, false, 1, 920, 644, 0, 0, 0, 0, 0, 0, 0, 134, 0, '', 0),
  (21, 31524, false, 1, 726, 744, 0, 0, 0, 0, 0, 0, 0, 60, 0, '', 0),
  (21, 31741, false, 1, 757, 773, 0, 0, 0, 0, 0, 0, 0, 76, 0, '', 0),
  (21, 31871, false, 1, 782, 561, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0),
  (21, 31872, false, 1, 774, 561, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0),
  (21, 31874, false, 1, 850, 561, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0),
  (21, 31875, false, 1, 858, 561, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0)
ON CONFLICT DO NOTHING;

-- Step 3: Sync template fields to match original server
UPDATE npc_template SET s_pid=100, by_type=0, s_level=6, s_size=100, i_selling_group=0, i_weapon_1=120150000, i_weapon_2=0 WHERE s_sid=150 AND is_monster=true;
UPDATE npc_template SET s_pid=100, by_type=0, s_level=16, s_size=150, i_selling_group=0, i_weapon_1=130250000, i_weapon_2=0 WHERE s_sid=155 AND is_monster=true;
UPDATE npc_template SET s_pid=100, by_type=0, s_level=20, s_size=150, i_selling_group=0, i_weapon_1=130450000, i_weapon_2=0 WHERE s_sid=159 AND is_monster=true;
UPDATE npc_template SET s_pid=200, by_type=0, s_level=11, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=250 AND is_monster=true;
UPDATE npc_template SET s_pid=200, by_type=0, s_level=13, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=251 AND is_monster=true;
UPDATE npc_template SET s_pid=200, by_type=0, s_level=17, s_size=200, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=253 AND is_monster=true;
UPDATE npc_template SET s_pid=200, by_type=0, s_level=19, s_size=180, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=254 AND is_monster=true;
UPDATE npc_template SET s_pid=210, by_type=0, s_level=40, s_size=90, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=256 AND is_monster=true;
UPDATE npc_template SET s_pid=210, by_type=0, s_level=20, s_size=90, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=257 AND is_monster=true;
UPDATE npc_template SET s_pid=300, by_type=0, s_level=16, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=350 AND is_monster=true;
UPDATE npc_template SET s_pid=300, by_type=0, s_level=18, s_size=150, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=351 AND is_monster=true;
UPDATE npc_template SET s_pid=300, by_type=0, s_level=30, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=353 AND is_monster=true;
UPDATE npc_template SET s_pid=300, by_type=0, s_level=24, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=354 AND is_monster=true;
UPDATE npc_template SET s_pid=400, by_type=0, s_level=34, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=450 AND is_monster=true;
UPDATE npc_template SET s_pid=400, by_type=0, s_level=38, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=452 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=23, s_size=120, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=550 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=35, s_size=120, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=551 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=38, s_size=140, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=552 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=47, s_size=140, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=553 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=35, s_size=150, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=554 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=40, s_size=150, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=555 AND is_monster=true;
UPDATE npc_template SET s_pid=30200, by_type=162, s_level=100, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=603 AND is_monster=false;
UPDATE npc_template SET s_pid=600, by_type=0, s_level=40, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=650 AND is_monster=true;
UPDATE npc_template SET s_pid=600, by_type=0, s_level=45, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=652 AND is_monster=true;
UPDATE npc_template SET s_pid=600, by_type=0, s_level=45, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=653 AND is_monster=true;
UPDATE npc_template SET s_pid=700, by_type=0, s_level=1, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=750 AND is_monster=true;
UPDATE npc_template SET s_pid=700, by_type=0, s_level=4, s_size=140, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=752 AND is_monster=true;
UPDATE npc_template SET s_pid=800, by_type=0, s_level=2, s_size=90, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=850 AND is_monster=true;
UPDATE npc_template SET s_pid=800, by_type=0, s_level=7, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=852 AND is_monster=true;
UPDATE npc_template SET s_pid=900, by_type=0, s_level=31, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=950 AND is_monster=true;
UPDATE npc_template SET s_pid=900, by_type=0, s_level=36, s_size=130, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=953 AND is_monster=true;
UPDATE npc_template SET s_pid=900, by_type=0, s_level=39, s_size=140, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=954 AND is_monster=true;
UPDATE npc_template SET s_pid=1000, by_type=0, s_level=55, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=1056 AND is_monster=true;
UPDATE npc_template SET s_pid=1000, by_type=0, s_level=35, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=1057 AND is_monster=true;
UPDATE npc_template SET s_pid=1000, by_type=0, s_level=40, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=1058 AND is_monster=true;
UPDATE npc_template SET s_pid=1100, by_type=0, s_level=45, s_size=150, i_selling_group=0, i_weapon_1=121510000, i_weapon_2=0 WHERE s_sid=1154 AND is_monster=true;
UPDATE npc_template SET s_pid=1100, by_type=0, s_level=30, s_size=100, i_selling_group=0, i_weapon_1=121510000, i_weapon_2=0 WHERE s_sid=1180 AND is_monster=true;
UPDATE npc_template SET s_pid=1601, by_type=0, s_level=30, s_size=120, i_selling_group=0, i_weapon_1=130250000, i_weapon_2=170150000 WHERE s_sid=1671 AND is_monster=true;
UPDATE npc_template SET s_pid=32021, by_type=197, s_level=30, s_size=80, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=3201 AND is_monster=true;
UPDATE npc_template SET s_pid=5001, by_type=24, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=5001 AND is_monster=false;
UPDATE npc_template SET s_pid=700, by_type=0, s_level=10, s_size=220, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8643 AND is_monster=true;
UPDATE npc_template SET s_pid=100, by_type=0, s_level=10, s_size=180, i_selling_group=0, i_weapon_1=120150000, i_weapon_2=0 WHERE s_sid=8644 AND is_monster=true;
UPDATE npc_template SET s_pid=800, by_type=0, s_level=10, s_size=170, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8645 AND is_monster=true;
UPDATE npc_template SET s_pid=300, by_type=0, s_level=30, s_size=170, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8646 AND is_monster=true;
UPDATE npc_template SET s_pid=200, by_type=0, s_level=20, s_size=230, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8647 AND is_monster=true;
UPDATE npc_template SET s_pid=900, by_type=0, s_level=60, s_size=220, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8648 AND is_monster=true;
UPDATE npc_template SET s_pid=500, by_type=0, s_level=40, s_size=200, i_selling_group=0, i_weapon_1=190450000, i_weapon_2=0 WHERE s_sid=8649 AND is_monster=true;
UPDATE npc_template SET s_pid=1000, by_type=0, s_level=40, s_size=200, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8650 AND is_monster=true;
UPDATE npc_template SET s_pid=600, by_type=0, s_level=50, s_size=200, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=8651 AND is_monster=true;
UPDATE npc_template SET s_pid=1100, by_type=0, s_level=50, s_size=180, i_selling_group=0, i_weapon_1=121510000, i_weapon_2=0 WHERE s_sid=8652 AND is_monster=true;
UPDATE npc_template SET s_pid=2001, by_type=0, s_level=100, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9021 AND is_monster=true;
UPDATE npc_template SET s_pid=32010, by_type=179, s_level=52, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9271 AND is_monster=true;
UPDATE npc_template SET s_pid=1000, by_type=0, s_level=52, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9272 AND is_monster=true;
UPDATE npc_template SET s_pid=5816, by_type=0, s_level=52, s_size=50, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9273 AND is_monster=true;
UPDATE npc_template SET s_pid=5814, by_type=0, s_level=52, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9274 AND is_monster=true;
UPDATE npc_template SET s_pid=6705, by_type=0, s_level=52, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=9275 AND is_monster=true;
UPDATE npc_template SET s_pid=1601, by_type=0, s_level=17, s_size=100, i_selling_group=0, i_weapon_1=110110000, i_weapon_2=0 WHERE s_sid=9641 AND is_monster=true;
UPDATE npc_template SET s_pid=11000, by_type=11, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=120310000, i_weapon_2=170410000 WHERE s_sid=11021 AND is_monster=false;
UPDATE npc_template SET s_pid=31200, by_type=21, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13001 AND is_monster=false;
UPDATE npc_template SET s_pid=31300, by_type=21, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13003 AND is_monster=false;
UPDATE npc_template SET s_pid=25240, by_type=21, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13005 AND is_monster=false;
UPDATE npc_template SET s_pid=12200, by_type=22, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13006 AND is_monster=false;
UPDATE npc_template SET s_pid=12100, by_type=22, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13007 AND is_monster=false;
UPDATE npc_template SET s_pid=31400, by_type=22, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13008 AND is_monster=false;
UPDATE npc_template SET s_pid=31600, by_type=34, s_level=150, s_size=140, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13009 AND is_monster=false;
UPDATE npc_template SET s_pid=30600, by_type=47, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=13013 AND is_monster=false;
UPDATE npc_template SET s_pid=2060, by_type=49, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13015 AND is_monster=false;
UPDATE npc_template SET s_pid=31600, by_type=45, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=13016 AND is_monster=false;
UPDATE npc_template SET s_pid=31100, by_type=77, s_level=80, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=14301 AND is_monster=false;
UPDATE npc_template SET s_pid=2050, by_type=78, s_level=70, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=14401 AND is_monster=false;
UPDATE npc_template SET s_pid=21610, by_type=136, s_level=80, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=15002 AND is_monster=false;
UPDATE npc_template SET s_pid=11100, by_type=29, s_level=90, s_size=110, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16047 AND is_monster=false;
UPDATE npc_template SET s_pid=30090, by_type=109, s_level=90, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16073 AND is_monster=false;
UPDATE npc_template SET s_pid=30094, by_type=110, s_level=90, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16074 AND is_monster=false;
UPDATE npc_template SET s_pid=30400, by_type=126, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16079 AND is_monster=false;
UPDATE npc_template SET s_pid=31000, by_type=132, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16085 AND is_monster=false;
UPDATE npc_template SET s_pid=30700, by_type=31, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16096 AND is_monster=false;
UPDATE npc_template SET s_pid=30700, by_type=31, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=16097 AND is_monster=false;
UPDATE npc_template SET s_pid=31900, by_type=46, s_level=70, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=18004 AND is_monster=false;
UPDATE npc_template SET s_pid=30500, by_type=46, s_level=70, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=18005 AND is_monster=false;
UPDATE npc_template SET s_pid=31539, by_type=46, s_level=70, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=18034 AND is_monster=false;
UPDATE npc_template SET s_pid=12100, by_type=46, s_level=40, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19002 AND is_monster=false;
UPDATE npc_template SET s_pid=2050, by_type=46, s_level=79, s_size=100, i_selling_group=0, i_weapon_1=910119310, i_weapon_2=910119310 WHERE s_sid=19003 AND is_monster=false;
UPDATE npc_template SET s_pid=21510, by_type=46, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19004 AND is_monster=false;
UPDATE npc_template SET s_pid=19005, by_type=46, s_level=80, s_size=130, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19005 AND is_monster=false;
UPDATE npc_template SET s_pid=11610, by_type=46, s_level=70, s_size=100, i_selling_group=0, i_weapon_1=159302419, i_weapon_2=0 WHERE s_sid=19006 AND is_monster=false;
UPDATE npc_template SET s_pid=2050, by_type=46, s_level=79, s_size=100, i_selling_group=0, i_weapon_1=910119310, i_weapon_2=910119310 WHERE s_sid=19007 AND is_monster=false;
UPDATE npc_template SET s_pid=21510, by_type=46, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19008 AND is_monster=false;
UPDATE npc_template SET s_pid=11510, by_type=46, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19009 AND is_monster=false;
UPDATE npc_template SET s_pid=31121, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19010 AND is_monster=false;
UPDATE npc_template SET s_pid=31133, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19011 AND is_monster=false;
UPDATE npc_template SET s_pid=31131, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19012 AND is_monster=false;
UPDATE npc_template SET s_pid=31142, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19013 AND is_monster=false;
UPDATE npc_template SET s_pid=31146, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19014 AND is_monster=false;
UPDATE npc_template SET s_pid=31151, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19015 AND is_monster=false;
UPDATE npc_template SET s_pid=31155, by_type=46, s_level=10, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19016 AND is_monster=false;
UPDATE npc_template SET s_pid=30800, by_type=46, s_level=1, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19017 AND is_monster=false;
UPDATE npc_template SET s_pid=22200, by_type=31, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19018 AND is_monster=false;
UPDATE npc_template SET s_pid=25500, by_type=0, s_level=3, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19019 AND is_monster=false;
UPDATE npc_template SET s_pid=21510, by_type=46, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19020 AND is_monster=false;
UPDATE npc_template SET s_pid=11100, by_type=22, s_level=40, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19021 AND is_monster=false;
UPDATE npc_template SET s_pid=2050, by_type=22, s_level=40, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19022 AND is_monster=false;
UPDATE npc_template SET s_pid=31300, by_type=21, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19023 AND is_monster=false;
UPDATE npc_template SET s_pid=26002, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19052 AND is_monster=false;
UPDATE npc_template SET s_pid=26001, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19053 AND is_monster=false;
UPDATE npc_template SET s_pid=2060, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19054 AND is_monster=false;
UPDATE npc_template SET s_pid=12100, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19055 AND is_monster=false;
UPDATE npc_template SET s_pid=22200, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19056 AND is_monster=false;
UPDATE npc_template SET s_pid=12200, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19057 AND is_monster=false;
UPDATE npc_template SET s_pid=26006, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19060 AND is_monster=false;
UPDATE npc_template SET s_pid=26005, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19063 AND is_monster=false;
UPDATE npc_template SET s_pid=26005, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19064 AND is_monster=false;
UPDATE npc_template SET s_pid=26002, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=19066 AND is_monster=false;
UPDATE npc_template SET s_pid=12201, by_type=174, s_level=80, s_size=100, i_selling_group=0, i_weapon_1=910024328, i_weapon_2=0 WHERE s_sid=19073 AND is_monster=false;
UPDATE npc_template SET s_pid=20004, by_type=74, s_level=120, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=20004 AND is_monster=false;
UPDATE npc_template SET s_pid=20005, by_type=74, s_level=120, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=20005 AND is_monster=false;
UPDATE npc_template SET s_pid=21000, by_type=11, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=21021 AND is_monster=false;
UPDATE npc_template SET s_pid=30900, by_type=21, s_level=60, s_size=130, i_selling_group=0, i_weapon_1=0, i_weapon_2=182110000 WHERE s_sid=22301 AND is_monster=false;
UPDATE npc_template SET s_pid=21000, by_type=146, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=121110000, i_weapon_2=171510000 WHERE s_sid=24414 AND is_monster=false;
UPDATE npc_template SET s_pid=28000, by_type=46, s_level=100, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25000 AND is_monster=false;
UPDATE npc_template SET s_pid=31122, by_type=46, s_level=100, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25001 AND is_monster=false;
UPDATE npc_template SET s_pid=31300, by_type=46, s_level=100, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25002 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=112, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25008 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=113, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25009 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=114, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25010 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=111, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25151 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=112, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25152 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=113, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25153 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=114, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25154 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=111, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25155 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=112, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25156 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=113, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25157 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=114, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25158 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=111, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25159 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=112, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25160 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=113, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25161 AND is_monster=false;
UPDATE npc_template SET s_pid=30300, by_type=114, s_level=60, s_size=70, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25162 AND is_monster=false;
UPDATE npc_template SET s_pid=2050, by_type=126, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=910119310, i_weapon_2=910119310 WHERE s_sid=25174 AND is_monster=false;
UPDATE npc_template SET s_pid=32036, by_type=132, s_level=60, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25177 AND is_monster=false;
UPDATE npc_template SET s_pid=14002, by_type=46, s_level=70, s_size=50, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25181 AND is_monster=false;
UPDATE npc_template SET s_pid=14002, by_type=46, s_level=70, s_size=50, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25182 AND is_monster=false;
UPDATE npc_template SET s_pid=14002, by_type=46, s_level=70, s_size=50, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25183 AND is_monster=false;
UPDATE npc_template SET s_pid=14002, by_type=46, s_level=70, s_size=50, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=25184 AND is_monster=false;
UPDATE npc_template SET s_pid=2100, by_type=174, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=29028 AND is_monster=false;
UPDATE npc_template SET s_pid=2060, by_type=132, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=29056 AND is_monster=false;
UPDATE npc_template SET s_pid=30700, by_type=21, s_level=80, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=29057 AND is_monster=false;
UPDATE npc_template SET s_pid=19005, by_type=174, s_level=50, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=29079 AND is_monster=false;
UPDATE npc_template SET s_pid=20571, by_type=126, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=29235 AND is_monster=false;
UPDATE npc_template SET s_pid=5853, by_type=174, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31402 AND is_monster=false;
UPDATE npc_template SET s_pid=19005, by_type=174, s_level=50, s_size=120, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31508 AND is_monster=false;
UPDATE npc_template SET s_pid=31100, by_type=174, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31511 AND is_monster=false;
UPDATE npc_template SET s_pid=31400, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31524 AND is_monster=false;
UPDATE npc_template SET s_pid=30700, by_type=174, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31525 AND is_monster=false;
UPDATE npc_template SET s_pid=11100, by_type=46, s_level=5, s_size=100, i_selling_group=0, i_weapon_1=128431369, i_weapon_2=128431369 WHERE s_sid=31526 AND is_monster=false;
UPDATE npc_template SET s_pid=20521, by_type=46, s_level=60, s_size=150, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31719 AND is_monster=false;
UPDATE npc_template SET s_pid=31600, by_type=46, s_level=100, s_size=130, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31720 AND is_monster=false;
UPDATE npc_template SET s_pid=31200, by_type=46, s_level=90, s_size=110, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31741 AND is_monster=false;
UPDATE npc_template SET s_pid=9948, by_type=174, s_level=50, s_size=120, i_selling_group=0, i_weapon_1=1111468747, i_weapon_2=1111472747 WHERE s_sid=31772 AND is_monster=false;
UPDATE npc_template SET s_pid=30001, by_type=174, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=0, i_weapon_2=0 WHERE s_sid=31774 AND is_monster=false;
UPDATE npc_template SET s_pid=26005, by_type=82, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31870 AND is_monster=false;
UPDATE npc_template SET s_pid=26005, by_type=83, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31871 AND is_monster=false;
UPDATE npc_template SET s_pid=26005, by_type=84, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31872 AND is_monster=false;
UPDATE npc_template SET s_pid=26002, by_type=85, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31873 AND is_monster=false;
UPDATE npc_template SET s_pid=26002, by_type=86, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31874 AND is_monster=false;
UPDATE npc_template SET s_pid=26002, by_type=87, s_level=50, s_size=100, i_selling_group=0, i_weapon_1=130650000, i_weapon_2=170450000 WHERE s_sid=31875 AND is_monster=false;

-- Summary: 4 templates + 180 spawns + 165 updates
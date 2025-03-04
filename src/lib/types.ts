export interface PotentialDuplicate {
  file_path1: string;
  file_path2: string;
  distance: number;
  size1: number;
  size2: number;
  resolution1: [number, number];
  resolution2: [number, number];
  format1: string;
  format2: string;
}

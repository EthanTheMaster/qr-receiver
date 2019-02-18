/* tslint:disable */
export class Block {
  free(): void;
}
export class QrReceiver {
  free(): void;
  static new(): QrReceiver;
  process(arg0: Uint8Array): void;
  get_progress_percentage(): number;
  get_finished_data(): Uint8Array;
  has_completed_download(): boolean;
  get_num_pending_blocks(): number;
}
export class XorShift {
  free(): void;
}

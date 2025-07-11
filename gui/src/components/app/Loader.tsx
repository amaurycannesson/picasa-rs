import { LoaderPinwheelIcon } from 'lucide-react';

export const Loader = ({ message }: { message: string }) => {
  return (
    <div className="flex">
      <LoaderPinwheelIcon className="animate-spin" />
      <span className="pl-1">{message}</span>
    </div>
  );
};

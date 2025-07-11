import { ChevronDownIcon } from 'lucide-react';
import * as React from 'react';

import { Button } from '@/components/ui/button';
import { Calendar } from '@/components/ui/calendar';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';

interface DatePickerProps {
  value: string | undefined;
  onChange: (value: string | undefined) => void;
  placeholder?: string;
}

export const DatePicker = ({ value, onChange, placeholder = 'Select date' }: DatePickerProps) => {
  const [open, setOpen] = React.useState(false);
  const [date, setDate] = React.useState<Date | undefined>(value ? new Date(value) : undefined);

  React.useEffect(() => {
    setDate(value ? new Date(value) : undefined);
  }, [value]);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          type="button"
          variant="outline"
          className="text-muted-foreground w-full justify-between font-normal"
        >
          {date ? date.toLocaleDateString() : placeholder}
          <ChevronDownIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-auto overflow-hidden p-0" align="start">
        <Calendar
          mode="single"
          selected={date}
          captionLayout="dropdown"
          onSelect={(selectedDate) => {
            setDate(selectedDate);
            onChange(selectedDate ? selectedDate.toISOString() : undefined);
            setOpen(false);
          }}
        />
      </PopoverContent>
    </Popover>
  );
};

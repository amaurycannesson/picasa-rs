import { CheckIcon, ChevronsUpDownIcon } from 'lucide-react';
import * as React from 'react';

import { Person } from '@/bindings';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { FormControl, FormItem } from '@/components/ui/form';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { cn, createSmartFilter } from '@/lib/utils';

interface PersonComboboxProps extends React.ComponentProps<"div"> {
  field: {
    value: string | undefined;
    onChange: (value: string) => void;
  };
  persons: Person[];
  placeholder?: string;
}

export function PersonCombobox({ field, persons, placeholder = 'Select person...', ...props }: PersonComboboxProps) {
  const [personOpen, setPersonOpen] = React.useState(false);

  return (
    <FormItem {...props}>
      <Popover open={personOpen} onOpenChange={setPersonOpen}>
        <FormControl>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              aria-expanded={personOpen}
              className="w-full justify-between"
            >
              {field.value ? (
                persons.find((person) => String(person.id) === field.value)?.name ||
                `Person ${field.value}`
              ) : (
                <span className="text-muted-foreground">{placeholder}</span>
              )}
              <ChevronsUpDownIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
        </FormControl>
        <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0">
          <Command filter={createSmartFilter(persons, (person) => person.name)}>
            <CommandInput placeholder="Search person..." />
            <CommandList>
              <CommandEmpty>No person found.</CommandEmpty>
              <CommandGroup>
                {persons.map((person) => (
                  <CommandItem
                    key={person.id}
                    value={String(person.id)}
                    onSelect={(currentValue) => {
                      field.onChange(currentValue === field.value ? '' : currentValue);
                      setPersonOpen(false);
                    }}
                  >
                    <CheckIcon
                      className={cn(
                        'mr-2 h-4 w-4',
                        field.value === String(person.id) ? 'opacity-100' : 'opacity-0',
                      )}
                    />
                    {person.name}
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>
    </FormItem>
  );
}
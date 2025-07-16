import { ChevronDown } from 'lucide-react';
import { matchSorter } from 'match-sorter';
import * as React from 'react';

import { Person } from '@/bindings';
import {
  Combobox,
  ComboboxAnchor,
  ComboboxBadgeItem,
  ComboboxBadgeList,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxTrigger,
} from '@/components/ui/combobox';
import { FormControl, FormItem } from '@/components/ui/form';

interface PersonComboboxProps extends React.ComponentProps<'div'> {
  field: {
    value: string | undefined;
    onChange: (value: string) => void;
  };
  persons: Person[];
  placeholder?: string;
}

interface PeopleComboboxProps extends React.ComponentProps<'div'> {
  field: {
    value: string[] | undefined;
    onChange: (value: string[]) => void;
  };
  persons: Person[];
  placeholder?: string;
}

const usePersonFilter = (persons: Person[]) => {
  return (options: string[], inputValue: string) => {
    const filteredPersons = persons.filter((person) => options.includes(String(person.id)));

    return matchSorter(filteredPersons, inputValue, {
      keys: ['name'],
      threshold: matchSorter.rankings.MATCHES,
    }).map((person) => String(person.id));
  };
};

const PersonOptions = ({ persons }: { persons: Person[] }) => (
  <>
    <ComboboxEmpty>No person found.</ComboboxEmpty>
    {persons.map((person) => (
      <ComboboxItem key={person.id} value={String(person.id)}>
        {person.name}
      </ComboboxItem>
    ))}
  </>
);

export function PersonCombobox({
  field,
  persons,
  placeholder = 'Select person...',
  ...props
}: PersonComboboxProps) {
  const onFilter = usePersonFilter(persons);

  return (
    <FormItem {...props}>
      <Combobox
        value={field.value || ''}
        onValueChange={(value) => field.onChange(value)}
        onFilter={onFilter}
        multiple={false}
        autoHighlight={false}
      >
        <FormControl>
          <ComboboxAnchor>
            <ComboboxInput placeholder={placeholder} className="flex-1" />
            <ComboboxTrigger>
              <ChevronDown className="h-4 w-4" />
            </ComboboxTrigger>
          </ComboboxAnchor>
        </FormControl>
        <ComboboxContent>
          <PersonOptions persons={persons} />
        </ComboboxContent>
      </Combobox>
    </FormItem>
  );
}

export function PeopleCombobox({
  field,
  persons,
  placeholder = 'Select people...',
  ...props
}: PeopleComboboxProps) {
  const onFilter = usePersonFilter(persons);

  return (
    <FormItem {...props}>
      <Combobox
        value={field.value || []}
        onValueChange={(value) => field.onChange(value)}
        onFilter={onFilter}
        multiple={true}
        autoHighlight={true}
      >
        <FormControl>
          <ComboboxAnchor className="h-full flex-wrap px-3 py-2">
            <ComboboxBadgeList>
              {(field.value || []).map((personId) => {
                const person = persons.find((p) => String(p.id) === personId);
                if (!person) return null;

                return (
                  <ComboboxBadgeItem key={personId} value={personId}>
                    {person.name}
                  </ComboboxBadgeItem>
                );
              })}
            </ComboboxBadgeList>
            <ComboboxInput placeholder={placeholder} className="h-auto min-w-20 flex-1" />
            <ComboboxTrigger className="absolute top-3 right-2">
              <ChevronDown className="h-4 w-4" />
            </ComboboxTrigger>
          </ComboboxAnchor>
        </FormControl>
        <ComboboxContent>
          <PersonOptions persons={persons} />
        </ComboboxContent>
      </Combobox>
    </FormItem>
  );
}

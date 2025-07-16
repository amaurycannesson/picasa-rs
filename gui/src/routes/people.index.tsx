import { zodResolver } from '@hookform/resolvers/zod';
import { Await, createFileRoute, Link, useRouter } from '@tanstack/react-router';
import { CheckIcon, Loader2Icon } from 'lucide-react';
import * as React from 'react';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';

import { commands, PendingFaceReview, Person, Result } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { FaceCrop } from '@/components/app/FaceCrop';
import { PersonCombobox } from '@/components/app/PersonCombobox';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardFooter } from '@/components/ui/card';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from '@/components/ui/carousel';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Separator } from '@/components/ui/separator';
import { DEFAULT_PHOTO_SEARCH } from '@/photoSearch';

export const Route = createFileRoute('/people/')({
  component: RouteComponent,
  loader: async () => {
    const people = await commands.listPersons();
    const pendingReviewsPromise = commands.getPendingManualReviews();

    if (people.status === 'error') throw new Error(people.error);

    return {
      people: people.data,
      pendingReviewsPromise,
    };
  },
  errorComponent: ErrorMessage,
});

function RouteComponent() {
  const { pendingReviewsPromise, people } = Route.useLoaderData();
  const router = useRouter();

  const handleCreatePerson = async (face: PendingFaceReview, personName: string) => {
    const result = await commands.createPersonFromFaces(personName, face.face_ids);

    if (result.status === 'ok') {
      toast.success(`${personName} has been created`);
      router.invalidate();
    } else {
      toast.error(`Failed to create new person: ${result.error}`);
    }
  };

  const handleAssignFaces = async (face: PendingFaceReview, personId: string) => {
    const result = await commands.assignPersonToFaces(face.face_ids, Number(personId));

    if (result.status === 'ok') {
      const person = people?.find((p) => p.id === parseInt(personId));
      toast.success(`Faces assigned to ${person?.name || 'person'}`);
      router.invalidate();
    } else {
      toast.error(`Failed to assign faces: ${result.error}`);
    }
  };

  return (
    <div>
      <Await
        promise={pendingReviewsPromise}
        fallback={
          <div className="flex pb-4">
            <Loader2Icon className="animate-spin" />
            <span className="pl-1">Looking for new faces...</span>
          </div>
        }
      >
        {(res: Result<PendingFaceReview[], string>) =>
          res.status === 'ok' &&
          res.data.length > 0 && (
            <PendingReview faces={res.data}>
              {(face) => (
                <CardReview
                  key={face.cluster_id}
                  face={face}
                  people={people}
                  onCreatePerson={(personName) => handleCreatePerson(face, personName)}
                  onAssignFaces={(personId) => handleAssignFaces(face, personId)}
                />
              )}
            </PendingReview>
          )
        }
      </Await>
      <People people={people} />
    </div>
  );
}

const People = ({ people }: { people: Person[] | null }) => {
  return (
    <div>
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">People</h4>
      {(!people || people.length === 0) && <>No people found.</>}
      <div className="flex flex-wrap gap-2">
        {people && people.map((person) => <_Person key={person.id} person={person} />)}
      </div>
    </div>
  );
};

const _Person = ({ person }: { person: Person }) => {
  return (
    <div className="flex flex-col items-center space-y-2 py-2">
      <Link
        to={`/people/$id/gallery`}
        params={{ id: person.id.toString() }}
        search={DEFAULT_PHOTO_SEARCH}
      >
        <Avatar className="size-16">
          <AvatarFallback className="text-2xl">
            {person.name.charAt(0).toUpperCase()}
          </AvatarFallback>
        </Avatar>
        <span className="text-center">{person.name}</span>
      </Link>
    </div>
  );
};

const personNameSchema = z.object({
  personName: z.string().min(3, 'Person name must be at least 3 characters'),
});

const assignFacesSchema = z.object({
  personId: z.string().min(1, 'Please select a person'),
});

type PersonNameFormValues = z.infer<typeof personNameSchema>;
type AssignFacesFormValues = z.infer<typeof assignFacesSchema>;

const CardReview = ({
  face,
  people,
  onCreatePerson,
  onAssignFaces,
}: {
  face: PendingFaceReview;
  people: Person[];
  onCreatePerson: (personName: string) => Promise<void>;
  onAssignFaces: (personId: string) => Promise<void>;
}) => {
  const [isCreateLoading, setIsCreateLoading] = useState(false);
  const [isAssignLoading, setIsAssignLoading] = useState(false);

  const form = useForm<PersonNameFormValues>({
    resolver: zodResolver(personNameSchema),
    defaultValues: {
      personName: '',
    },
  });

  const assignForm = useForm<AssignFacesFormValues>({
    resolver: zodResolver(assignFacesSchema),
    defaultValues: {
      personId: '',
    },
  });

  const handleCreatePerson = async (values: PersonNameFormValues) => {
    if (values.personName) {
      setIsCreateLoading(true);
      await onCreatePerson(values.personName);
      setIsCreateLoading(false);
      form.reset();
    }
  };

  const handleAssignFaces = async (values: AssignFacesFormValues) => {
    if (values.personId) {
      setIsAssignLoading(true);
      await onAssignFaces(values.personId);
      setIsAssignLoading(false);
      assignForm.reset();
    }
  };

  return (
    <Card className="relative w-1/3 px-2 py-4">
      <Carousel
        opts={{
          align: 'start',
        }}
        className="w-full"
      >
        <CarouselContent className="pl-2">
          {face.face_ids.map((id) => (
            <CarouselItem key={id} className="basis-1/3">
              <FaceCrop faceId={id} />
            </CarouselItem>
          ))}
        </CarouselContent>
        {face.face_ids.length > 3 && (
          <>
            <CarouselPrevious className="absolute top-1/2 left-2 -translate-y-1/2" />
            <CarouselNext className="absolute top-1/2 right-2 -translate-y-1/2" />
          </>
        )}
      </Carousel>
      <CardFooter className="flex flex-col space-y-2 p-0">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(handleCreatePerson)} className="flex w-full">
            <FormField
              control={form.control}
              name="personName"
              render={({ field }) => (
                <FormItem className="w-full">
                  <FormControl>
                    <Input placeholder="Enter person name" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              type="submit"
              className="ml-2"
              disabled={!form.formState.isValid || isCreateLoading}
            >
              {isCreateLoading ? <Loader2Icon className="animate-spin" /> : <CheckIcon />}
            </Button>
          </form>
        </Form>
        <div className="flex items-center w-full">
          <Separator className="flex-1" />
          <span className="px-4 text-sm text-muted-foreground">OR</span>
          <Separator className="flex-1" />
        </div>
        <Form {...assignForm}>
          <form onSubmit={assignForm.handleSubmit(handleAssignFaces)} className="flex w-full">
            <FormField
              control={assignForm.control}
              name="personId"
              render={({ field }) => (
                <PersonCombobox className="w-full" field={field} persons={people || []} />
              )}
            />
            <Button
              type="submit"
              className="ml-2"
              disabled={!assignForm.formState.isValid || isAssignLoading}
            >
              {isAssignLoading ? <Loader2Icon className="animate-spin" /> : <CheckIcon />}
            </Button>
          </form>
        </Form>
      </CardFooter>
    </Card>
  );
};

interface PendingReviewProps {
  faces: PendingFaceReview[];
  children: (face: PendingFaceReview) => React.ReactNode;
}

const PendingReview = ({ faces, children }: PendingReviewProps) => {
  return (
    <div className="pb-4">
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
        Pending review <Badge>{faces.length}</Badge>
      </h4>
      <div className="pt-2">{faces.map((face) => children(face))}</div>
    </div>
  );
};

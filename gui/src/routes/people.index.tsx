import { zodResolver } from '@hookform/resolvers/zod';
import { Await, createFileRoute, Link, useRouter } from '@tanstack/react-router';
import { CheckIcon, Loader2Icon } from 'lucide-react';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';

import { commands, PendingFaceReview, Person, Result } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { FaceCrop } from '@/components/app/FaceCrop';
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
          res.status === 'ok' && res.data.length > 0 && <PendingReview faces={res.data} />
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

type PersonNameFormValues = z.infer<typeof personNameSchema>;

const CardReview = ({
  face,
  onCreatePerson,
}: {
  face: PendingFaceReview;
  onCreatePerson: (personName: string) => Promise<void>;
}) => {
  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<PersonNameFormValues>({
    resolver: zodResolver(personNameSchema),
    defaultValues: {
      personName: '',
    },
  });

  const handleCreatePerson = async (values: PersonNameFormValues) => {
    if (values.personName) {
      setIsLoading(true);
      await onCreatePerson(values.personName);
      setIsLoading(false);
      form.reset();
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
      <CardFooter className="p-0">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(handleCreatePerson)} className="flex">
            <FormField
              control={form.control}
              name="personName"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input placeholder="Enter person name" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="ml-2" disabled={!form.formState.isValid || isLoading}>
              {isLoading ? <Loader2Icon className="animate-spin" /> : <CheckIcon />}
            </Button>
          </form>
        </Form>
      </CardFooter>
    </Card>
  );
};

const PendingReview = ({ faces }: { faces: PendingFaceReview[] }) => {
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

  return (
    <div className="pb-4">
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
        Pending review <Badge>{faces.length}</Badge>
      </h4>
      <div className="pt-2">
        {faces.map((face) => (
          <CardReview
            key={face.cluster_id}
            face={face}
            onCreatePerson={(personName) => handleCreatePerson(face, personName)}
          />
        ))}
      </div>
    </div>
  );
};

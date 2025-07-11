import { AlertCircleIcon } from 'lucide-react';

import { Alert, AlertDescription, AlertTitle } from '../ui/alert';

export const ErrorMessage = ({ error }: { error: { message: string } }) => {
  return (
    <Alert variant="destructive">
      <AlertCircleIcon />
      <AlertTitle>Error while loading page</AlertTitle>
      <AlertDescription>
        <p>{error.message}</p>
      </AlertDescription>
    </Alert>
  );
};

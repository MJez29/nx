import { NameAndDirectoryFormat } from '@nx/devkit/src/generators/artifact-name-and-directory-utils';

export interface Schema {
  directory?: string;
  name: string;
  description?: string;
  unitTestRunner: 'jest' | 'none';
  skipLintChecks?: boolean;
  skipFormat?: boolean;
  nameAndDirectoryFormat?: NameAndDirectoryFormat;

  /**
   * @deprecated Provide the `directory` option instead. The project will be determined from the directory provided. It will be removed in Nx v18.
   */
  project?: string;
}

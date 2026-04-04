import { stat, readdir } from 'fs/promises';
import type { Stats } from 'fs';
import {
    FileStat,
    FileSystemProvider as CSSFileSystemProvider,
    FileType
} from 'vscode-css-languageservice';
import { urlToPath } from '../../utils';

interface StatLike {
    isDirectory(): boolean;
    isFile(): boolean;
    isSymbolicLink(): boolean;
}

export class FileSystemProvider implements CSSFileSystemProvider {

    constructor() {
        this.readDirectory = this.readDirectory.bind(this);
        this.stat = this.stat.bind(this);
    }

    async stat(uri: string): Promise<FileStat> {
        const path = urlToPath(uri);

        if (!path) {
            return this.unknownStat();
        }

        let stat: Stats;
        try {
            stat = await stat(path);
        } catch (error) {
            if (
                error != null &&
                typeof error === 'object' &&
                'code' in error &&
                (error as { code: string }).code === 'ENOENT'
            ) {
                return {
                    type: FileType.Unknown,
                    ctime: -1,
                    mtime: -1,
                    size: -1
                };
            }

            throw error;
        }

        return {
            ctime: stat.ctimeMs,
            mtime: stat.mtimeMs,
            size: stat.size,
            type: this.getFileType(stat)
        };
    }

    private unknownStat(): FileStat {
        return {
            type: FileType.Unknown,
            ctime: -1,
            mtime: -1,
            size: -1
        };
    }

    private getFileType(stat: StatLike) {
        return stat.isDirectory()
            ? FileType.Directory
            : stat.isFile()
            ? FileType.File
            : stat.isSymbolicLink()
            ? FileType.SymbolicLink
            : FileType.Unknown;
    }

    async readDirectory(uri: string): Promise<Array<[string, FileType]>> {
        const path = urlToPath(uri);

        if (!path) {
            return [];
        }

        const files = await readdir(path, {
            withFileTypes: true
        });

        return files.map((file) => [file.name, this.getFileType(file)]);
    }
}

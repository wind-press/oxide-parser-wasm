<?php

declare(strict_types=1);

namespace WindPress\OxideParser;

use Exception;
use FFI;

class Parser
{
    /**
     * FFI instance.
     */
    private $ffi;

    /**
     * Stores the instance, implementing a Singleton pattern.
     */
    private static self $instance;

    /**
     * The Singleton's constructor should always be private to prevent direct
     * construction calls with the `new` operator.
     */
    private function __construct() {
        $os = PHP_OS_FAMILY;
        $lib_path = dirname(__DIR__) . '/ext';
        switch ($os) {
            case 'Windows':
                $lib_path .= '/win/oxide_parser_wasm.dll';
                break;
            case 'Darwin':
                $lib_path .= '/macos/liboxide_parser_wasm.dylib';
                break;
            default:
                $lib_path .= '/linux/liboxide_parser_wasm.so';
        }

        $this->ffi = FFI::cdef("
            char** find_tw_candidates_ffi(const char* input, size_t input_len, int loose);
            void free_candidates(char** ptr);
        ", $lib_path);
    }

    /**
     * Singletons should not be cloneable.
     */
    private function __clone() {}

    /**
     * Singletons should not be restorable from strings.
     *
     * @throws Exception Cannot unserialize a singleton.
     */
    public function __wakeup()
    {
        throw new Exception('Cannot unserialize a singleton.');
    }

    /**
     * This is the static method that controls the access to the singleton
     * instance. On the first run, it creates a singleton object and places it
     * into the static property. On subsequent runs, it returns the client existing
     * object stored in the static property.
     */
    public static function get_instance(): self
    {
        if (! isset(self::$instance)) {
            self::$instance = new self();
        }

        return self::$instance;
    }

    /**
     * Find tailwind candidates.
     *
     * @param string $input
     * @param int $loose 1 (true) or 0 (false)
     * @return array
     */
    public function find_tw_candidates(string $input, int $loose = 1): array
    {
        $candidates = $this->ffi->find_tw_candidates_ffi($input, strlen($input), $loose);
        $result = [];

        for ($i = 0; $candidates[$i] !== null; $i++) {
            $result[] = FFI::string($candidates[$i]);
        }

        $this->free_candidates($candidates);

        return $result;
    }

    /**
     * Free candidates.
     */
    private function free_candidates($candidates): void
    {
        $this->ffi->free_candidates($candidates);
    }
}

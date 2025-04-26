from typing import final, type_check_only

@final
@type_check_only
class _version_info(tuple[int, int, int, str, int]):
    if sys.version_info >= (3, 10):
        __match_args__: Final = ("major", "minor", "micro", "releaselevel", "serial")

    @property
    def major(self) -> int: ...
    @property
    def minor(self) -> int: ...
    @property
    def micro(self) -> int: ...
    @property
    def releaselevel(self) -> str: ...
    @property
    def serial(self) -> int: ...

version_info: _version_info

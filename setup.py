import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setuptools.setup(
    name="sysadmin",
    version="0.0.1",
    author="t-wilkinson",
    author_email="winston.trey.wilkinson@gmail.com",
    description="Packages to help with sys admin",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/t-wilkinson/sysadmin",
    # entry_points={
    #     "console_scripts": ['sysadmin = sysadmin.normalize'],
    # },
    scripts=['scripts/sysadmin'],
    project_urls={
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
    ],
    package_dir={"": "sysadmin"},
    packages=setuptools.find_packages(where="sysadmin"),
    python_requires=">=3.6",
)
